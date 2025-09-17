#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use counter::Counter;
pub use progress::Progress;

mod counter;
mod progress;

/// A convenience module that re-exports commonly used items.
///
/// This module is intended to be glob-imported for convenience:
#[doc(hidden)]
pub mod prelude {
    pub use super::{Counter, Progress};
}
