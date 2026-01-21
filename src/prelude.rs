//! Convenience re-exports for glob imports.
//!
//! This module provides a quick way to import the most commonly used types:
//!
//! ```rust
//! use countio::prelude::*;
//!
//! let counter = Counter::new(Vec::<u8>::new());
//! let progress = Progress::with_expected_writer_bytes(Vec::<u8>::new(), 100);
//! ```

pub use crate::{Counter, Progress};
