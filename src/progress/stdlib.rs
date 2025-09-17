use std::io::{BufRead, Read, Seek, Write};
use std::io::{Result, SeekFrom};

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
    fn reader() -> Result<()> {
        let reader = "Hello World!".as_bytes();
        let mut reader = Progress::new(reader);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf)?;

        assert_eq!(len, reader.bytes_read());
        assert_eq!(len as u128, reader.bytes_processed());

        Ok(())
    }

    #[test]
    fn buf_reader() -> Result<()> {
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
    fn writer() -> Result<()> {
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
    fn progress_with_known_total() -> Result<()> {
        let mut progress = Progress::with_total(Vec::new(), 100);
        progress.write_all(b"Hello")?; // 5 bytes

        assert_eq!(progress.percentage(), Some(0.05));
        assert_eq!(progress.bytes_written(), 5);
        assert_eq!(progress.bytes_processed(), 5);

        Ok(())
    }
}
