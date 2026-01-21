#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod stdlib;

#[cfg(feature = "futures")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
mod futures;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
mod tokio;

/// A wrapper that adds byte counting functionality to any reader, writer, or seeker.
///
/// `Counter<D>` tracks the total number of bytes read from and written to the underlying
/// I/O object, providing methods to query these statistics at any time.
///
/// # Type Parameter
///
/// * `D` - The underlying I/O object (reader, writer, or seeker)
///
/// # Examples
///
/// ## Basic Usage with `std::io`
///
/// ```rust
/// use std::io::{Read, Write};
/// use countio::Counter;
///
/// // Counting bytes read
/// let data = b"Hello, World!";
/// let mut reader = Counter::new(&data[..]);
/// let mut buffer = [0u8; 5];
/// reader.read(&mut buffer).unwrap();
/// assert_eq!(reader.reader_bytes(), 5);
///
/// // Counting bytes written
/// let mut writer = Counter::new(Vec::new());
/// writer.write_all(b"Hello").unwrap();
/// assert_eq!(writer.writer_bytes(), 5);
/// ```
///
/// ## With Buffered I/O
///
/// ```rust
/// use std::io::{BufRead, BufReader, BufWriter, Write};
/// use countio::Counter;
///
/// // Buffered reading
/// let data = "Line 1\nLine 2\nLine 3\n";
/// let reader = BufReader::new(data.as_bytes());
/// let mut counter = Counter::new(reader);
/// let mut line = String::new();
/// counter.read_line(&mut line).unwrap();
/// assert_eq!(counter.reader_bytes(), 7);
///
/// // Buffered writing
/// let writer = BufWriter::new(Vec::new());
/// let mut counter = Counter::new(writer);
/// counter.write_all(b"Hello, World!").unwrap();
/// counter.flush().unwrap();
/// assert_eq!(counter.writer_bytes(), 13);
/// ```
///
/// # Performance
///
/// The overhead of byte counting is minimal - just an integer addition per I/O operation.
/// The wrapper implements all relevant traits (`Read`, `Write`, `Seek`, etc.) by
/// delegating to the inner object while tracking byte counts.
pub struct Counter<D> {
    pub(crate) inner: D,
    pub(crate) reader_bytes: usize,
    pub(crate) writer_bytes: usize,
}

impl<D> Counter<D> {
    /// Creates a new `Counter<D>` wrapping the given I/O object with zero read/written bytes.
    ///
    /// # Arguments
    ///
    /// * `inner` - The I/O object to wrap (reader, writer, seeker, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Counter;
    /// use std::io::Cursor;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let cursor = Cursor::new(data);
    /// let counter = Counter::new(cursor);
    ///
    /// assert_eq!(counter.reader_bytes(), 0);
    /// assert_eq!(counter.writer_bytes(), 0);
    /// ```
    #[inline]
    pub const fn new(inner: D) -> Self {
        Self::with_bytes(inner, 0, 0)
    }

    /// Creates a new `Counter<D>` with pre-existing read/written byte counts.
    ///
    /// This is useful when you want to continue counting from a previous session
    /// or when wrapping an I/O object that has already processed some data.
    ///
    /// # Arguments
    ///
    /// * `inner` - The I/O object to wrap
    /// * `reader_bytes` - Initial count of bytes read
    /// * `writer_bytes` - Initial count of bytes written
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Counter;
    /// use std::io::Cursor;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let cursor = Cursor::new(data);
    /// let counter = Counter::with_bytes(cursor, 100, 50);
    ///
    /// assert_eq!(counter.reader_bytes(), 100);
    /// assert_eq!(counter.writer_bytes(), 50);
    /// ```
    #[inline]
    pub const fn with_bytes(inner: D, reader_bytes: usize, writer_bytes: usize) -> Self {
        Self {
            inner,
            reader_bytes,
            writer_bytes,
        }
    }

    /// Returns the total number of bytes read from the underlying I/O object.
    ///
    /// This count includes all bytes successfully read through any of the
    /// `Read` or `BufRead` trait methods.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Read;
    /// use countio::Counter;
    ///
    /// let data = b"Hello, World!";
    /// let mut reader = Counter::new(&data[..]);
    /// let mut buffer = [0u8; 5];
    ///
    /// reader.read_exact(&mut buffer).unwrap();
    /// assert_eq!(reader.reader_bytes(), 5);
    /// assert_eq!(reader.writer_bytes(), 0);
    /// ```
    #[inline]
    pub const fn reader_bytes(&self) -> usize {
        self.reader_bytes
    }

    /// Returns the total number of bytes written to the underlying I/O object.
    ///
    /// This count includes all bytes successfully written through any of the
    /// `Write` trait methods.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Write;
    /// use countio::Counter;
    ///
    /// let mut writer = Counter::new(Vec::new());
    /// writer.write_all(b"Hello").unwrap();
    /// writer.write_all(b", World!").unwrap();
    ///
    /// assert_eq!(writer.writer_bytes(), 13);
    /// assert_eq!(writer.reader_bytes(), 0);
    /// ```
    #[inline]
    pub const fn writer_bytes(&self) -> usize {
        self.writer_bytes
    }

    /// Returns the total number of bytes processed (read + written) as a `u128`.
    ///
    /// This is the sum of all bytes that have been read from or written to
    /// the underlying I/O object since the `Counter` was created. Using `u128`
    /// prevents overflow issues that could occur with large byte counts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::{Read, Write};
    /// use countio::Counter;
    ///
    /// let mut counter = Counter::new(Vec::new());
    /// counter.write_all(b"Hello").unwrap();
    ///
    /// let data = b"World";
    /// let mut reader = Counter::new(&data[..]);
    /// let mut buf = [0u8; 5];
    /// reader.read(&mut buf).unwrap();
    ///
    /// assert_eq!(counter.total_bytes(), 5);
    /// assert_eq!(reader.total_bytes(), 5);
    /// ```
    #[inline]
    pub const fn total_bytes(&self) -> u128 {
        (self.reader_bytes as u128) + (self.writer_bytes as u128)
    }

    /// Consumes the `Counter<D>` and returns the underlying I/O object.
    ///
    /// This is useful when you need to recover the original I/O object,
    /// perhaps to pass it to code that doesn't know about the `Counter` wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Write;
    /// use countio::Counter;
    ///
    /// let original_writer = Vec::new();
    /// let mut counter = Counter::new(original_writer);
    /// counter.write_all(b"Hello").unwrap();
    ///
    /// let recovered_writer = counter.into_inner();
    /// assert_eq!(recovered_writer, b"Hello");
    /// ```
    #[inline]
    pub fn into_inner(self) -> D {
        self.inner
    }

    /// Gets a reference to the underlying I/O object.
    ///
    /// This allows you to access the wrapped object's methods directly
    /// without consuming the `Counter`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Counter;
    /// use std::io::Cursor;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let cursor = Cursor::new(data.clone());
    /// let counter = Counter::new(cursor);
    ///
    /// assert_eq!(counter.get_ref().position(), 0);
    /// ```
    #[inline]
    pub const fn get_ref(&self) -> &D {
        &self.inner
    }

    /// Gets a mutable reference to the underlying I/O object.
    ///
    /// This allows you to modify the wrapped object directly. Note that
    /// any bytes read or written through the direct reference will not
    /// be counted by the `Counter`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use countio::Counter;
    /// use std::io::Cursor;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let cursor = Cursor::new(data);
    /// let mut counter = Counter::new(cursor);
    ///
    /// counter.get_mut().set_position(2);
    /// assert_eq!(counter.get_ref().position(), 2);
    /// ```
    #[inline]
    pub const fn get_mut(&mut self) -> &mut D {
        &mut self.inner
    }

    /// Resets the byte counters to zero without affecting the underlying I/O object.
    ///
    /// This is useful when you want to start counting from a fresh state
    /// without recreating the wrapper or losing the underlying object's state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::{Read, Write};
    /// use countio::Counter;
    ///
    /// let mut counter = Counter::new(Vec::new());
    /// counter.write_all(b"Hello").unwrap();
    /// assert_eq!(counter.writer_bytes(), 5);
    ///
    /// counter.reset();
    /// assert_eq!(counter.writer_bytes(), 0);
    /// assert_eq!(counter.reader_bytes(), 0);
    ///
    /// // The underlying data is preserved
    /// assert_eq!(counter.get_ref(), b"Hello");
    /// ```
    #[inline]
    pub const fn reset(&mut self) {
        self.reader_bytes = 0;
        self.writer_bytes = 0;
    }
}

impl<D> From<D> for Counter<D> {
    #[inline]
    fn from(inner: D) -> Self {
        Self::new(inner)
    }
}

impl<D: Clone> Clone for Counter<D> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            reader_bytes: self.reader_bytes,
            writer_bytes: self.writer_bytes,
        }
    }
}

impl<D: Default> Default for Counter<D> {
    fn default() -> Self {
        Self::new(D::default())
    }
}

impl<D: core::fmt::Debug> core::fmt::Debug for Counter<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Counter")
            .field("inner", &self.inner)
            .field("read", &self.reader_bytes)
            .field("written", &self.writer_bytes)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner() {
        let writer = vec![8u8];
        assert_eq!(writer.len(), 1);

        let mut writer = Counter::new(writer);
        writer.get_mut().clear();
        assert_eq!(writer.get_ref().len(), 0);

        let writer = writer.into_inner();
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn test_from() {
        let _: Counter<_> = Vec::<u8>::new().into();
    }

    #[test]
    fn test_with_bytes_creates_counter_with_initial_counts() {
        let counter = Counter::with_bytes(Vec::<u8>::new(), 100, 200);
        assert_eq!(counter.reader_bytes(), 100);
        assert_eq!(counter.writer_bytes(), 200);
        assert_eq!(counter.total_bytes(), 300);
    }

    #[test]
    fn test_reset() {
        use std::io::Write;

        let mut counter = Counter::new(Vec::new());
        counter.write_all(b"Hello").unwrap();
        assert_eq!(counter.writer_bytes(), 5);

        counter.reset();
        assert_eq!(counter.writer_bytes(), 0);
        assert_eq!(counter.reader_bytes(), 0);
        assert_eq!(counter.total_bytes(), 0);

        // Data is preserved
        assert_eq!(counter.get_ref(), b"Hello");
    }

    #[test]
    fn test_clone() {
        use std::io::Write;

        let mut counter = Counter::new(Vec::new());
        counter.write_all(b"Hello").unwrap();

        let cloned = counter.clone();
        assert_eq!(cloned.writer_bytes(), 5);
        assert_eq!(cloned.get_ref(), b"Hello");
    }

    #[test]
    fn test_default() {
        let counter: Counter<Vec<u8>> = Counter::default();
        assert_eq!(counter.reader_bytes(), 0);
        assert_eq!(counter.writer_bytes(), 0);
        assert!(counter.get_ref().is_empty());
    }
}
