//! `thriller-kernels` is a Rust wrapper for Macro Kernels along with an API interface.

#![deny(warnings)]
#![deny(missing_docs)]

mod copy;
mod sync;

pub use copy::Copy;
pub use sync::Sync;
