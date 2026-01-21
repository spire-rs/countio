#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod stdlib;

#[cfg(feature = "futures")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
mod futures;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
mod tokio;

use crate::Counter;

/// A progress tracker that extends `Counter` with percentage completion tracking.
///
/// `Progress<D>` wraps a `Counter<D>` and adds tracking for expected total bytes
/// and derived metrics useful for progress reporting. Supports separate tracking
/// for read and write operations.
///
/// # Examples
///
/// ```rust
/// use countio::Progress;
/// use std::io::Write;
///
/// let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 1024);
/// progress.write_all(b"Hello").unwrap();
///
/// assert_eq!(progress.total_bytes(), 5);
/// assert_eq!(progress.expected_writer_bytes(), Some(1024));
/// assert!(progress.writer_percentage().unwrap() < 1.0);
/// ```
pub struct Progress<D> {
    counter: Counter<D>,
    expected_reader_bytes: Option<usize>,
    expected_writer_bytes: Option<usize>,
}

impl<D> Progress<D> {
    /// Creates a new `Progress<D>` with unknown expected sizes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    ///
    /// let progress = Progress::new(Vec::<u8>::new());
    /// assert_eq!(progress.expected_reader_bytes(), None);
    /// assert_eq!(progress.expected_writer_bytes(), None);
    /// ```
    pub const fn new(inner: D) -> Self {
        Self {
            counter: Counter::new(inner),
            expected_reader_bytes: None,
            expected_writer_bytes: None,
        }
    }

    /// Creates a new `Progress<D>` with a known expected read size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    ///
    /// let progress = Progress::with_expected_reader_bytes(&b"hello"[..], 5);
    /// assert_eq!(progress.expected_reader_bytes(), Some(5));
    /// assert_eq!(progress.expected_writer_bytes(), None);
    /// ```
    pub const fn with_expected_reader_bytes(inner: D, expected: usize) -> Self {
        Self {
            counter: Counter::new(inner),
            expected_reader_bytes: Some(expected),
            expected_writer_bytes: None,
        }
    }

    /// Creates a new `Progress<D>` with a known expected write size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    ///
    /// let progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), 1024);
    /// assert_eq!(progress.expected_reader_bytes(), None);
    /// assert_eq!(progress.expected_writer_bytes(), Some(1024));
    /// ```
    pub const fn with_expected_writer_bytes(inner: D, expected: usize) -> Self {
        Self {
            counter: Counter::new(inner),
            expected_reader_bytes: None,
            expected_writer_bytes: Some(expected),
        }
    }

    /// Creates a new `Progress<D>` with known expected read and write sizes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    /// use std::io::Cursor;
    ///
    /// let cursor = Cursor::new(vec![0u8; 100]);
    /// let progress = Progress::with_expected_bytes(cursor, 100, 50);
    /// assert_eq!(progress.expected_reader_bytes(), Some(100));
    /// assert_eq!(progress.expected_writer_bytes(), Some(50));
    /// ```
    pub const fn with_expected_bytes(
        inner: D,
        expected_reader_bytes: usize,
        expected_writer_bytes: usize,
    ) -> Self {
        Self {
            counter: Counter::new(inner),
            expected_reader_bytes: Some(expected_reader_bytes),
            expected_writer_bytes: Some(expected_writer_bytes),
        }
    }

    /// Returns the total number of bytes processed (read + written).
    #[inline]
    pub const fn total_bytes(&self) -> u128 {
        self.counter.total_bytes()
    }

    /// Returns the number of bytes read.
    #[inline]
    pub const fn reader_bytes(&self) -> usize {
        self.counter.reader_bytes()
    }

    /// Returns the number of bytes written.
    #[inline]
    pub const fn writer_bytes(&self) -> usize {
        self.counter.writer_bytes()
    }

    /// Returns the expected number of bytes to read, if known.
    #[inline]
    pub const fn expected_reader_bytes(&self) -> Option<usize> {
        self.expected_reader_bytes
    }

    /// Returns the expected number of bytes to write, if known.
    #[inline]
    pub const fn expected_writer_bytes(&self) -> Option<usize> {
        self.expected_writer_bytes
    }

    /// Sets the expected number of bytes to read.
    pub const fn set_expected_reader_bytes(&mut self, expected: Option<usize>) {
        self.expected_reader_bytes = expected;
    }

    /// Sets the expected number of bytes to write.
    pub const fn set_expected_writer_bytes(&mut self, expected: Option<usize>) {
        self.expected_writer_bytes = expected;
    }

    /// Returns the read completion percentage (0.0 to 1.0) if expected read size is known.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    /// use std::io::Read;
    ///
    /// let data = b"Hello, World!";
    /// let mut progress = Progress::with_expected_reader_bytes(&data[..], 13);
    /// let mut buf = [0u8; 5];
    /// progress.read(&mut buf).unwrap();
    ///
    /// assert!((progress.reader_percentage().unwrap() - 0.3846).abs() < 0.001);
    /// ```
    #[allow(clippy::cast_precision_loss)]
    pub fn reader_percentage(&self) -> Option<f64> {
        self.expected_reader_bytes.map(|expected| {
            if expected == 0 {
                1.0
            } else {
                (self.reader_bytes() as f64) / (expected as f64)
            }
        })
    }

    /// Returns the write completion percentage (0.0 to 1.0) if expected write size is known.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    /// use std::io::Write;
    ///
    /// let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
    /// progress.write_all(b"Hello").unwrap();
    ///
    /// assert_eq!(progress.writer_percentage(), Some(0.05));
    /// ```
    #[allow(clippy::cast_precision_loss)]
    pub fn writer_percentage(&self) -> Option<f64> {
        self.expected_writer_bytes.map(|expected| {
            if expected == 0 {
                1.0
            } else {
                (self.writer_bytes() as f64) / (expected as f64)
            }
        })
    }

    /// Consumes the `Progress<D>` and returns the underlying I/O object.
    #[inline]
    pub fn into_inner(self) -> D {
        self.counter.into_inner()
    }

    /// Gets a reference to the underlying I/O object.
    #[inline]
    pub const fn get_ref(&self) -> &D {
        self.counter.get_ref()
    }

    /// Gets a mutable reference to the underlying I/O object.
    #[inline]
    pub const fn get_mut(&mut self) -> &mut D {
        self.counter.get_mut()
    }

    /// Resets the byte counters to zero without affecting the underlying I/O object
    /// or the expected totals.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Write;
    /// use countio::Progress;
    ///
    /// let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
    /// progress.write_all(b"Hello").unwrap();
    /// assert_eq!(progress.writer_bytes(), 5);
    /// assert_eq!(progress.writer_percentage(), Some(0.05));
    ///
    /// progress.reset();
    /// assert_eq!(progress.writer_bytes(), 0);
    /// assert_eq!(progress.writer_percentage(), Some(0.0));
    /// assert_eq!(progress.expected_writer_bytes(), Some(100)); // Expected preserved
    /// ```
    #[inline]
    pub const fn reset(&mut self) {
        self.counter.reset();
    }
}

impl<D> From<Counter<D>> for Progress<D> {
    fn from(counter: Counter<D>) -> Self {
        Self {
            counter,
            expected_reader_bytes: None,
            expected_writer_bytes: None,
        }
    }
}

impl<D: Clone> Clone for Progress<D> {
    fn clone(&self) -> Self {
        Self {
            counter: self.counter.clone(),
            expected_reader_bytes: self.expected_reader_bytes,
            expected_writer_bytes: self.expected_writer_bytes,
        }
    }
}

impl<D: Default> Default for Progress<D> {
    fn default() -> Self {
        Self::new(D::default())
    }
}

impl<D: core::fmt::Debug> core::fmt::Debug for Progress<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Progress")
            .field("counter", &self.counter)
            .field("expected_reader_bytes", &self.expected_reader_bytes)
            .field("expected_writer_bytes", &self.expected_writer_bytes)
            .field("reader_percentage", &self.reader_percentage())
            .field("writer_percentage", &self.writer_percentage())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Result, Write};

    use super::*;

    #[test]
    fn test_progress_basic() {
        let progress = Progress::new(Vec::<u8>::new());
        assert_eq!(progress.total_bytes(), 0);
        assert_eq!(progress.expected_reader_bytes(), None);
        assert_eq!(progress.expected_writer_bytes(), None);

        let progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), 1000);
        assert_eq!(progress.expected_writer_bytes(), Some(1000));
        assert_eq!(progress.expected_reader_bytes(), None);

        let progress = Progress::with_expected_reader_bytes(&b"test"[..], 4);
        assert_eq!(progress.expected_reader_bytes(), Some(4));
        assert_eq!(progress.expected_writer_bytes(), None);
    }

    #[test]
    fn test_progress_with_expected_bytes() {
        use std::io::Cursor;

        let cursor = Cursor::new(vec![0u8; 100]);
        let progress = Progress::with_expected_bytes(cursor, 100, 50);
        assert_eq!(progress.expected_reader_bytes(), Some(100));
        assert_eq!(progress.expected_writer_bytes(), Some(50));
    }

    #[test]
    fn test_progress_writer_percentage() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        progress.write_all(b"Hello")?;

        assert_eq!(progress.writer_percentage(), Some(0.05));

        let zero_progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), 0);
        assert_eq!(zero_progress.writer_percentage(), Some(1.0));

        let unknown_progress = Progress::new(Vec::<u8>::new());
        assert_eq!(unknown_progress.writer_percentage(), None);

        Ok(())
    }

    #[test]
    fn test_progress_reader_percentage() -> Result<()> {
        let data = b"Hello, World!";
        let mut progress = Progress::with_expected_reader_bytes(&data[..], 13);
        let mut buf = [0u8; 5];
        progress.read_exact(&mut buf)?;

        let pct = progress.reader_percentage().unwrap();
        assert!((pct - 5.0 / 13.0).abs() < 0.0001);

        Ok(())
    }

    #[test]
    fn test_progress_from_counter() {
        let counter = Counter::new(Vec::<u8>::new());
        let progress = Progress::from(counter);
        assert_eq!(progress.total_bytes(), 0);
        assert_eq!(progress.expected_reader_bytes(), None);
        assert_eq!(progress.expected_writer_bytes(), None);
    }

    #[test]
    fn test_progress_set_expected() {
        let mut progress = Progress::new(Vec::<u8>::new());
        assert_eq!(progress.expected_writer_bytes(), None);
        assert_eq!(progress.writer_percentage(), None);

        progress.set_expected_writer_bytes(Some(100));
        assert_eq!(progress.expected_writer_bytes(), Some(100));
        assert_eq!(progress.writer_percentage(), Some(0.0));

        progress.set_expected_writer_bytes(None);
        assert_eq!(progress.expected_writer_bytes(), None);
        assert_eq!(progress.writer_percentage(), None);

        progress.set_expected_reader_bytes(Some(50));
        assert_eq!(progress.expected_reader_bytes(), Some(50));
    }

    #[test]
    fn test_progress_percentage_edge_cases() -> Result<()> {
        let progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), 0);
        assert_eq!(progress.writer_percentage(), Some(1.0));

        let progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), usize::MAX);
        assert_eq!(progress.writer_percentage(), Some(0.0));

        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), usize::MAX);
        progress.write_all(b"x")?;
        let percentage = progress.writer_percentage().unwrap();
        assert!(percentage > 0.0 && percentage < 0.000_000_1);

        Ok(())
    }

    #[test]
    fn test_progress_from_counter_with_existing_data() -> Result<()> {
        let mut counter = Counter::new(Vec::new());
        counter.write_all(b"existing")?;

        let progress = Progress::from(counter);
        assert_eq!(progress.total_bytes(), 8);
        assert_eq!(progress.writer_bytes(), 8);
        assert_eq!(progress.expected_writer_bytes(), None);

        Ok(())
    }

    #[test]
    fn test_progress_large_byte_counts() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 1000);

        for _ in 0..100 {
            progress.write_all(b"1234567890")?;
        }

        assert_eq!(progress.writer_bytes(), 1000);
        assert_eq!(progress.total_bytes(), 1000);
        assert_eq!(progress.writer_percentage(), Some(1.0));

        Ok(())
    }

    #[test]
    fn test_progress_reset() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        progress.write_all(b"Hello")?;
        assert_eq!(progress.writer_bytes(), 5);

        progress.reset();
        assert_eq!(progress.writer_bytes(), 0);
        assert_eq!(progress.writer_percentage(), Some(0.0));
        assert_eq!(progress.expected_writer_bytes(), Some(100));

        Ok(())
    }

    #[test]
    fn test_progress_clone() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        progress.write_all(b"Hello")?;

        let cloned = progress.clone();
        assert_eq!(cloned.writer_bytes(), 5);
        assert_eq!(cloned.expected_writer_bytes(), Some(100));

        Ok(())
    }

    #[test]
    fn test_progress_default() {
        let progress: Progress<Vec<u8>> = Progress::default();
        assert_eq!(progress.reader_bytes(), 0);
        assert_eq!(progress.writer_bytes(), 0);
        assert_eq!(progress.expected_reader_bytes(), None);
        assert_eq!(progress.expected_writer_bytes(), None);
    }

    #[test]
    fn test_progress_debug() -> Result<()> {
        let mut progress = Progress::with_expected_writer_bytes(Vec::new(), 100);
        progress.write_all(b"test")?;

        let debug_str = format!("{progress:?}");
        assert!(debug_str.contains("Progress"));
        assert!(debug_str.contains("written"));
        assert!(debug_str.contains("expected_writer_bytes"));

        Ok(())
    }
}
