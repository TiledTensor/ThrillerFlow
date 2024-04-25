//! ThrillerFlow is a DataFlow Analyise and Codegen Framework written in Rust.

#![deny(warnings)]
#![deny(missing_docs)]
mod buffer;
mod dataflow;
mod id;
mod task;

pub use buffer::Buffer;
pub use dataflow::{ThrillerGraph, ThrillerNode};

/// Initialize the ThrillerFlow framework.
pub fn initialize() {
    let id_counter = id::IdCounter::new();
    unsafe {
        id::ID_COUNTER.get_or_init(|| id_counter);
    }
}
