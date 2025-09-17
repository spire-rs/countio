use std::io::{Result, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

use crate::Progress;

impl<R: AsyncRead + Unpin> AsyncRead for Progress<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.poll_read(ctx, buf)
    }
}

impl<R: AsyncBufRead + Unpin> AsyncBufRead for Progress<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<&[u8]>> {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.poll_fill_buf(ctx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.consume(amt);
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for Progress<W> {
    fn poll_write(self: Pin<&mut Self>, ctx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.poll_write(ctx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.poll_flush(ctx)
    }

    fn poll_close(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.poll_close(ctx)
    }
}

impl<D: AsyncSeek + Unpin> AsyncSeek for Progress<D> {
    fn poll_seek(self: Pin<&mut Self>, ctx: &mut Context<'_>, pos: SeekFrom) -> Poll<Result<u64>> {
        let progress = self.get_mut();
        let pin = Pin::new(&mut progress.counter);
        pin.poll_seek(ctx, pos)
    }
}

#[cfg(test)]
mod test {
    use std::io::Result;

    use futures_util::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;

    #[futures_test::test]
    async fn test_reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let mut reader = Progress::new(reader);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf).await?;

        assert_eq!(len, reader.bytes_read());
        assert_eq!(len as u128, reader.bytes_processed());

        Ok(())
    }

    #[futures_test::test]
    async fn test_buf_reader() -> Result<()> {
        use futures_util::io::{AsyncBufReadExt, BufReader};

        let reader = "Hello World!".as_bytes();
        let reader = BufReader::new(reader);
        let mut reader = Progress::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).await?;

        assert_eq!(len, reader.bytes_read());
        assert_eq!(len as u128, reader.bytes_processed());

        Ok(())
    }

    #[futures_test::test]
    async fn test_writer() -> Result<()> {
        use futures_util::io::BufWriter;

        let writer = Vec::new();
        let writer = BufWriter::new(writer);
        let mut writer = Progress::new(writer);

        let buf = "Hello World!".as_bytes();
        let len = writer.write(buf).await?;
        writer.flush().await?;

        assert_eq!(len, writer.bytes_written());
        assert_eq!(len as u128, writer.bytes_processed());

        Ok(())
    }

    #[futures_test::test]
    async fn test_progress_with_known_total() -> Result<()> {
        let mut progress = Progress::with_total(Vec::new(), 100);
        progress.write_all(b"Hello").await?;

        assert_eq!(progress.percentage(), Some(0.05));
        assert_eq!(progress.bytes_written(), 5);
        assert_eq!(progress.bytes_processed(), 5);

        Ok(())
    }

    #[futures_test::test]
    async fn test_zero_byte_ops() -> Result<()> {
        use futures_util::io::{AsyncReadExt, AsyncWriteExt};

        let mut progress = Progress::with_total(Vec::new(), 100);

        // Zero byte write
        let len = progress.write(&[]).await?;
        assert_eq!(len, 0);
        assert_eq!(progress.bytes_written(), 0);
        assert_eq!(progress.percentage(), Some(0.0));

        // Zero byte read
        let reader = "".as_bytes();
        let mut reader = Progress::new(reader);
        let mut buf = [0u8; 10];
        let len = reader.read(&mut buf).await?;
        assert_eq!(len, 0);
        assert_eq!(reader.bytes_read(), 0);

        Ok(())
    }
}
