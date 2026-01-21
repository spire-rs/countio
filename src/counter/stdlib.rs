use std::io::{BufRead, Read, Result, Seek, SeekFrom, Write};

use crate::Counter;

impl<R: Read> Read for Counter<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.inner.read(buf)?;
        self.reader_bytes += bytes;
        Ok(bytes)
    }
}

impl<R: BufRead> BufRead for Counter<R> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader_bytes += amt;
        self.inner.consume(amt);
    }
}

impl<W: Write> Write for Counter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.inner.write(buf)?;
        self.writer_bytes += bytes;
        Ok(bytes)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl<D: Seek> Seek for Counter<D> {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.inner.seek(pos)
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufReader, BufWriter};

    use super::*;

    #[test]
    fn test_reader() -> Result<()> {
        let mut reader = Counter::new(&b"Hello World!"[..]);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf)?;

        assert_eq!(len, reader.reader_bytes());
        assert_eq!(len as u128, reader.total_bytes());

        Ok(())
    }

    #[test]
    fn test_buf_reader() -> Result<()> {
        let reader = BufReader::new(&b"Hello World!"[..]);
        let mut reader = Counter::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf)?;

        assert_eq!(len, reader.reader_bytes());
        assert_eq!(len as u128, reader.total_bytes());

        Ok(())
    }

    #[test]
    fn test_writer() -> Result<()> {
        let writer = Vec::new();
        let writer = BufWriter::new(writer);
        let mut writer = Counter::new(writer);

        let len = writer.write(b"Hello World!")?;
        writer.flush()?;

        assert_eq!(len, writer.writer_bytes());
        assert_eq!(len as u128, writer.total_bytes());

        Ok(())
    }

    #[test]
    fn test_debug() {
        let writer = Vec::<u8>::new();
        let writer = Counter::new(writer);

        let fmt = "Counter { inner: [], read: 0, written: 0 }";
        assert_eq!(format!("{writer:?}"), fmt);
    }

    #[test]
    fn test_seek() -> Result<()> {
        use std::io::{Cursor, SeekFrom};

        let data = b"Hello, World!".to_vec();
        let cursor = Cursor::new(data);
        let mut counter = Counter::new(cursor);

        let pos = counter.seek(SeekFrom::Start(0))?;
        assert_eq!(pos, 0);

        let pos = counter.seek(SeekFrom::End(0))?;
        assert_eq!(pos, 13);

        let pos = counter.seek(SeekFrom::Current(-5))?;
        assert_eq!(pos, 8);

        assert_eq!(counter.reader_bytes(), 0);
        assert_eq!(counter.writer_bytes(), 0);
        assert_eq!(counter.total_bytes(), 0);

        Ok(())
    }

    #[test]
    fn test_zero_byte_ops() -> Result<()> {
        let mut counter = Counter::new(Vec::new());

        let len = counter.write(&[])?;
        assert_eq!(len, 0);
        assert_eq!(counter.writer_bytes(), 0);
        assert_eq!(counter.total_bytes(), 0);

        let mut reader = Counter::new(&b""[..]);
        let mut buf = [0u8; 10];
        let len = reader.read(&mut buf)?;
        assert_eq!(len, 0);
        assert_eq!(reader.reader_bytes(), 0);
        assert_eq!(reader.total_bytes(), 0);

        Ok(())
    }
}
