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
    use std::io::Cursor;

    use tokio::io::{
        AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter,
    };

    use super::*;

    #[tokio::test]
    async fn test_reader() -> Result<()> {
        let mut reader = Counter::new(&b"Hello World!"[..]);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf).await?;

        assert_eq!(len, reader.reader_bytes());

        Ok(())
    }

    #[tokio::test]
    async fn test_buf_reader() -> Result<()> {
        let reader = BufReader::new(&b"Hello World!"[..]);
        let mut reader = Counter::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).await?;

        assert_eq!(len, reader.reader_bytes());

        Ok(())
    }

    #[tokio::test]
    async fn test_writer() -> Result<()> {
        let writer = BufWriter::new(Vec::new());
        let mut writer = Counter::new(writer);

        let len = writer.write(b"Hello World!").await?;
        writer.flush().await?;

        assert_eq!(len, writer.writer_bytes());

        Ok(())
    }

    #[tokio::test]
    async fn test_seek() -> Result<()> {
        let data = b"Hello, World!".to_vec();
        let cursor = Cursor::new(data);
        let mut counter = Counter::new(cursor);

        let pos = counter.seek(SeekFrom::Start(0)).await?;
        assert_eq!(pos, 0);

        let pos = counter.seek(SeekFrom::End(0)).await?;
        assert_eq!(pos, 13);

        let pos = counter.seek(SeekFrom::Current(-5)).await?;
        assert_eq!(pos, 8);

        assert_eq!(counter.reader_bytes(), 0);
        assert_eq!(counter.writer_bytes(), 0);
        assert_eq!(counter.total_bytes(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_zero_byte_ops() -> Result<()> {
        let mut counter = Counter::new(Vec::new());
        let len = counter.write(&[]).await?;
        assert_eq!(len, 0);
        assert_eq!(counter.writer_bytes(), 0);
        assert_eq!(counter.total_bytes(), 0);

        let mut reader = Counter::new(&b""[..]);
        let mut buf = [0u8; 10];
        let len = reader.read(&mut buf).await?;
        assert_eq!(len, 0);
        assert_eq!(reader.reader_bytes(), 0);
        assert_eq!(reader.total_bytes(), 0);

        Ok(())
    }
}
