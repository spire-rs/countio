use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

use crate::Counter;

impl<R: AsyncRead + Unpin> AsyncRead for Counter<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);

        let poll = pin.poll_read(ctx, buf);
        if let Poll::Ready(Ok(bytes)) = poll {
            counter.reader_bytes += bytes;
        }

        poll
    }
}

impl<R: AsyncBufRead + Unpin> AsyncBufRead for Counter<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<&[u8]>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_fill_buf(ctx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let counter = self.get_mut();
        counter.reader_bytes += amt;
        let pin = Pin::new(&mut counter.inner);
        pin.consume(amt);
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for Counter<W> {
    fn poll_write(self: Pin<&mut Self>, ctx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);

        let poll = pin.poll_write(ctx, buf);
        if let Poll::Ready(Ok(bytes)) = poll {
            counter.writer_bytes += bytes;
        }

        poll
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_flush(ctx)
    }

    fn poll_close(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_close(ctx)
    }
}

impl<D: AsyncSeek + Unpin> AsyncSeek for Counter<D> {
    fn poll_seek(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        pos: std::io::SeekFrom,
    ) -> Poll<Result<u64>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_seek(ctx, pos)
    }
}

#[cfg(test)]
mod test {
    use futures_util::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;

    #[futures_test::test]
    async fn test_reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let mut reader = Counter::new(reader);

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
        let mut reader = Counter::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).await?;

        assert_eq!(len, reader.bytes_read());
        assert_eq!(reader.bytes_processed(), 12);

        Ok(())
    }

    #[futures_test::test]
    async fn test_writer() -> Result<()> {
        use futures_util::io::BufWriter;

        let writer = Vec::new();
        let writer = BufWriter::new(writer);
        let mut writer = Counter::new(writer);

        let buf = "Hello World!".as_bytes();
        let len = writer.write(buf).await?;
        writer.flush().await?;

        assert_eq!(len, writer.bytes_written());
        assert_eq!(writer.bytes_processed(), 12);

        Ok(())
    }

    #[futures_test::test]
    async fn test_zero_byte_ops() -> Result<()> {
        use futures_util::io::{AsyncReadExt, AsyncWriteExt};

        let mut counter = Counter::new(Vec::new());
        let len = counter.write(&[]).await?;

        assert_eq!(len, 0);
        assert_eq!(counter.bytes_written(), 0);
        assert_eq!(counter.bytes_processed(), 0);

        let reader = "".as_bytes();
        let mut reader = Counter::new(reader);
        let mut buf = [0u8; 10];
        let len = reader.read(&mut buf).await?;

        assert_eq!(len, 0);
        assert_eq!(reader.bytes_read(), 0);
        assert_eq!(reader.bytes_processed(), 0);

        Ok(())
    }
}
