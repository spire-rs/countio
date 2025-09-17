#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use counter::Counter;
pub use progress::Progress;

mod counter;
mod progress;

/// A convenience module that re-exports commonly used items.
///
/// This module is intended to be glob-imported for convenience:
///
/// ```rust
/// use countio::prelude::*;
///
/// let counter = Counter::new(Vec::<u8>::new());
/// ```
#[doc(hidden)]
pub mod prelude {
    pub use super::{Counter, Progress};
}
