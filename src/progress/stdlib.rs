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
    use std::io::{BufReader, BufWriter, Cursor};

    use super::*;

    #[test]
    fn test_reader() -> Result<()> {
        let mut reader = Progress::new(&b"Hello World!"[..]);

        let mut buf = Vec::new();
        let len = reader.read_to_end(&mut buf)?;

        assert_eq!(len, reader.reader_bytes());
        assert_eq!(len as u128, reader.total_bytes());

        Ok(())
    }

    #[test]
    fn test_buf_reader() -> Result<()> {
        let reader = BufReader::new(&b"Hello World!"[..]);
        let mut reader = Progress::new(reader);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf)?;

        assert_eq!(len, reader.reader_bytes());
        assert_eq!(len as u128, reader.total_bytes());

        Ok(())
    }

    #[test]
    fn test_writer() -> Result<()> {
        let writer = BufWriter::new(Vec::new());
        let mut writer = Progress::new(writer);

        let len = writer.write(b"Hello World!")?;
        writer.flush()?;

        assert_eq!(len, writer.writer_bytes());
        assert_eq!(len as u128, writer.total_bytes());

        Ok(())
    }

    #[test]
    fn test_seek() -> Result<()> {
        let data = b"Hello, World!".to_vec();
        let cursor = Cursor::new(data);
        let mut progress = Progress::with_expected_reader_bytes(cursor, 13);

        let pos = progress.seek(SeekFrom::Start(0))?;
        assert_eq!(pos, 0);

        let pos = progress.seek(SeekFrom::End(0))?;
        assert_eq!(pos, 13);

        let pos = progress.seek(SeekFrom::Current(-5))?;
        assert_eq!(pos, 8);

        assert_eq!(progress.reader_bytes(), 0);
        assert_eq!(progress.writer_bytes(), 0);
        assert_eq!(progress.total_bytes(), 0);

        Ok(())
    }

    #[test]
    fn test_progress_with_known_total() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        progress.write_all(b"Hello")?;

        assert_eq!(progress.writer_percentage(), Some(0.05));
        assert_eq!(progress.writer_bytes(), 5);
        assert_eq!(progress.total_bytes(), 5);

        Ok(())
    }

    #[test]
    fn test_progress_edge_cases() -> Result<()> {
        let zero_progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), 0);
        assert_eq!(zero_progress.writer_percentage(), Some(1.0));

        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        let len = progress.write(&[])?;
        assert_eq!(len, 0);
        assert_eq!(progress.writer_bytes(), 0);
        assert_eq!(progress.writer_percentage(), Some(0.0));

        Ok(())
    }

    #[test]
    fn test_zero_byte_ops() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        let len = progress.write(&[])?;
        assert_eq!(len, 0);
        assert_eq!(progress.writer_bytes(), 0);
        assert_eq!(progress.total_bytes(), 0);

        let mut reader = Progress::new(&b""[..]);
        let mut buf = [0u8; 10];
        let len = reader.read(&mut buf)?;
        assert_eq!(len, 0);
        assert_eq!(reader.reader_bytes(), 0);
        assert_eq!(reader.total_bytes(), 0);

        Ok(())
    }
}
