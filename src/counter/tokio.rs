use std::io::{Result, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};

use crate::Counter;

impl<R: AsyncRead + Unpin> AsyncRead for Counter<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        let bytes = buf.filled().len();

        let poll = pin.poll_read(ctx, buf);
        if matches!(poll, Poll::Ready(Ok(()))) {
            let bytes = buf.filled().len() - bytes;
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

    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_shutdown(ctx)
    }
}

impl<D: AsyncSeek + Unpin> AsyncSeek for Counter<D> {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> Result<()> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.start_seek(position)
    }

    fn poll_complete(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<u64>> {
        let counter = self.get_mut();
        let pin = Pin::new(&mut counter.inner);
        pin.poll_complete(ctx)
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

    use super::*;

    #[tokio::test]
    async fn test_reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let mut reader = Counter::new(reader);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf).await?;

        assert_eq!(len, reader.bytes_read());

        Ok(())
    }

    #[tokio::test]
    async fn test_buf_reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let reader = BufReader::new(reader);
        let mut reader = Counter::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).await?;

        assert_eq!(len, reader.bytes_read());

        Ok(())
    }

    #[tokio::test]
    async fn test_writer() -> Result<()> {
        let writer = Vec::new();
        let writer = BufWriter::new(writer);
        let mut writer = Counter::new(writer);

        let buf = "Hello World!".as_bytes();
        let len = writer.write(buf).await?;
        writer.flush().await?;

        assert_eq!(len, writer.bytes_written());

        Ok(())
    }

    #[tokio::test]
    async fn test_seek() -> Result<()> {
        use std::io::{Cursor, SeekFrom};

        use tokio::io::AsyncSeekExt;

        let data = b"Hello, World!".to_vec();
        let cursor = Cursor::new(data);
        let mut counter = Counter::new(cursor);

        let pos = counter.seek(SeekFrom::Start(0)).await?;
        assert_eq!(pos, 0);

        let pos = counter.seek(SeekFrom::End(0)).await?;
        assert_eq!(pos, 13);

        let pos = counter.seek(SeekFrom::Current(-5)).await?;
        assert_eq!(pos, 8);

        assert_eq!(counter.bytes_read(), 0);
        assert_eq!(counter.bytes_written(), 0);
        assert_eq!(counter.bytes_processed(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_zero_byte_ops() -> Result<()> {
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
