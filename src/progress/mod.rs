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
/// and derived metrics useful for progress reporting.
///
/// # Examples
///
/// ```rust
/// use countio::Progress;
/// use std::io::{Read, Write};
///
/// let mut progress = Progress::with_total(Vec::new(), 1024);
/// progress.write_all(b"Hello").unwrap();
///
/// assert_eq!(progress.bytes_processed(), 5);
/// assert_eq!(progress.total_expected(), Some(1024));
/// assert!(progress.percentage().unwrap() < 1.0);
/// ```
pub struct Progress<D> {
    counter: Counter<D>,
    total_expected: Option<u64>,
}

impl<D> Progress<D> {
    /// Creates a new `Progress<D>` with unknown total size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    ///
    /// let progress = Progress::new(Vec::<u8>::new());
    /// assert_eq!(progress.total_expected(), None);
    /// ```
    pub fn new(inner: D) -> Self {
        Self {
            counter: Counter::new(inner),
            total_expected: None,
        }
    }

    /// Creates a new `Progress<D>` with a known total expected size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    ///
    /// let progress = Progress::with_total(Vec::<u8>::new(), 2048);
    /// assert_eq!(progress.total_expected(), Some(2048));
    /// ```
    pub fn with_total(inner: D, total_expected: u64) -> Self {
        Self {
            counter: Counter::new(inner),
            total_expected: Some(total_expected),
        }
    }

    /// Returns the underlying `Counter`.
    pub fn counter(&self) -> &Counter<D> {
        &self.counter
    }

    /// Returns a mutable reference to the underlying `Counter`.
    pub fn counter_mut(&mut self) -> &mut Counter<D> {
        &mut self.counter
    }

    /// Returns the total number of bytes processed (read + written).
    pub fn bytes_processed(&self) -> u128 {
        self.counter.total_bytes()
    }

    /// Returns the number of bytes read.
    pub fn bytes_read(&self) -> usize {
        self.counter.reader_bytes()
    }

    /// Returns the number of bytes written.
    pub fn bytes_written(&self) -> usize {
        self.counter.writer_bytes()
    }

    /// Returns the expected total size, if known.
    pub fn total_expected(&self) -> Option<u64> {
        self.total_expected
    }

    /// Sets the expected total size.
    pub fn set_total_expected(&mut self, total: Option<u64>) {
        self.total_expected = total;
    }

    /// Returns the completion percentage (0.0 to 1.0) if total size is known.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Progress;
    /// use std::io::Write;
    ///
    /// let mut progress = Progress::with_total(Vec::new(), 100);
    /// progress.write_all(b"Hello").unwrap();
    ///
    /// assert_eq!(progress.percentage().unwrap(), 0.05);
    /// ```
    pub fn percentage(&self) -> Option<f64> {
        self.total_expected.map(|total| {
            if total == 0 {
                1.0
            } else {
                (self.bytes_processed() as f64) / (total as f64)
            }
        })
    }

    /// Consumes the `Progress<D>` and returns the underlying I/O object.
    pub fn into_inner(self) -> D {
        self.counter.into_inner()
    }

    /// Gets a reference to the underlying I/O object.
    pub fn get_ref(&self) -> &D {
        self.counter.get_ref()
    }

    /// Gets a mutable reference to the underlying I/O object.
    pub fn get_mut(&mut self) -> &mut D {
        self.counter.get_mut()
    }
}

impl<D> From<Counter<D>> for Progress<D> {
    fn from(counter: Counter<D>) -> Self {
        Self {
            counter,
            total_expected: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Result, Write};

    use super::*;

    #[test]
    fn progress_basic_functionality() {
        let progress = Progress::new(Vec::<u8>::new());
        assert_eq!(progress.bytes_processed(), 0);
        assert_eq!(progress.total_expected(), None);

        let progress_with_total = Progress::with_total(Vec::<u8>::new(), 1000);
        assert_eq!(progress_with_total.total_expected(), Some(1000));
    }

    #[test]
    fn progress_percentage_calculation() -> Result<()> {
        let mut progress = Progress::with_total(Vec::new(), 100);
        progress.write_all(b"Hello")?;

        assert_eq!(progress.percentage(), Some(0.05));

        let zero_progress = Progress::with_total(Vec::<u8>::new(), 0);
        assert_eq!(zero_progress.percentage(), Some(1.0));

        let unknown_progress = Progress::new(Vec::<u8>::new());
        assert_eq!(unknown_progress.percentage(), None);

        Ok(())
    }

    #[test]
    fn progress_from_counter() {
        let counter = Counter::new(Vec::<u8>::new());
        let progress = Progress::from(counter);
        assert_eq!(progress.bytes_processed(), 0);
        assert_eq!(progress.total_expected(), None);
    }
}
