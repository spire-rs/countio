use std::io::{BufRead, Read, Result, Seek, SeekFrom, Write};

use crate::Progress;

impl<R: Read> Read for Progress<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.counter.read(buf)
    }
}

impl<R: BufRead> BufRead for Progress<R> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.counter.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.counter.consume(amt);
    }
}

impl<W: Write> Write for Progress<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.counter.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.counter.flush()
    }
}

impl<D: Seek> Seek for Progress<D> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.counter.seek(pos)
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufReader, BufWriter};

    use super::*;

    #[test]
    fn test_reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let mut reader = Progress::new(reader);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf)?;

        assert_eq!(len, reader.bytes_read());
        assert_eq!(len as u128, reader.bytes_processed());

        Ok(())
    }

    #[test]
    fn test_buf_reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let reader = BufReader::new(reader);
        let mut reader = Progress::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf)?;

        assert_eq!(len, reader.bytes_read());
        assert_eq!(len as u128, reader.bytes_processed());

        Ok(())
    }

    #[test]
    fn test_writer() -> Result<()> {
        let writer = Vec::new();
        let writer = BufWriter::new(writer);
        let mut writer = Progress::new(writer);

        let buf = "Hello World!".as_bytes();
        let len = writer.write(buf)?;
        writer.flush()?;

        assert_eq!(len, writer.bytes_written());
        assert_eq!(len as u128, writer.bytes_processed());

        Ok(())
    }

    #[test]
    fn test_seek() -> Result<()> {
        use std::io::{Cursor, Seek, SeekFrom};

        let data = b"Hello, World!".to_vec();
        let cursor = Cursor::new(data);
        let mut progress = Progress::with_total(cursor, 13);

        let pos = progress.seek(SeekFrom::Start(0))?;
        assert_eq!(pos, 0);

        let pos = progress.seek(SeekFrom::End(0))?;
        assert_eq!(pos, 13);

        let pos = progress.seek(SeekFrom::Current(-5))?;
        assert_eq!(pos, 8);

        assert_eq!(progress.bytes_read(), 0);
        assert_eq!(progress.bytes_written(), 0);
        assert_eq!(progress.bytes_processed(), 0);

        Ok(())
    }

    #[test]
    fn test_progress_with_known_total() -> Result<()> {
        let mut progress = Progress::with_total(Vec::new(), 100);
        progress.write_all(b"Hello")?; // 5 bytes

        assert_eq!(progress.percentage(), Some(0.05));
        assert_eq!(progress.bytes_written(), 5);
        assert_eq!(progress.bytes_processed(), 5);

        Ok(())
    }

    #[test]
    fn test_progress_edge_cases() -> Result<()> {
        use std::io::Write;

        let zero_progress = Progress::with_total(Vec::<u8>::new(), 0);
        assert_eq!(zero_progress.percentage(), Some(1.0));

        let mut progress = Progress::with_total(Vec::new(), 100);
        let len = progress.write(&[])?;
        assert_eq!(len, 0);
        assert_eq!(progress.bytes_written(), 0);
        assert_eq!(progress.percentage(), Some(0.0));

        Ok(())
    }

    #[test]
    fn test_zero_byte_ops() -> Result<()> {
        use std::io::{Read, Write};

        let mut progress = Progress::with_total(Vec::new(), 100);
        let len = progress.write(&[])?;
        assert_eq!(len, 0);
        assert_eq!(progress.bytes_written(), 0);
        assert_eq!(progress.bytes_processed(), 0);

        let reader = "".as_bytes();
        let mut reader = Progress::new(reader);
        let mut buf = [0u8; 10];
        let len = reader.read(&mut buf)?;
        assert_eq!(len, 0);
        assert_eq!(reader.bytes_read(), 0);
        assert_eq!(reader.bytes_processed(), 0);

        Ok(())
    }
}
