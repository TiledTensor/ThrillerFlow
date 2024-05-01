//! ThrillerFlow is a DataFlow Analyise and Codegen Framework written in Rust.

#![deny(warnings)]
#![deny(missing_docs)]
mod access;
mod buffer;
mod dataflow;
mod id;
mod task;

pub use access::AccessMap;
pub use buffer::Buffer;
pub use dataflow::{
    AttachedEdge, BlockType, ThrillerBlock, ThrillerEdge, ThrillerGraph, ThrillerNode,
};
pub use task::Task;

use id::ID_COUNTER;

/// GPU Memory Level.
#[derive(Default)]
pub enum MemoryLevel {
    /// Register File
    #[default]
    Register,
    /// Shared Memory
    Shared,
    /// Global Memory
    Global,
}

/// Initialize the ThrillerFlow framework.
pub fn initialize() {
    let id_counter = id::IdCounter::new();
    unsafe {
        id::ID_COUNTER.get_or_init(|| id_counter);
    }
}

/// Return the next unique ID.
pub fn next_id() -> usize {
    unsafe { ID_COUNTER.get_mut().unwrap().next() }
}
