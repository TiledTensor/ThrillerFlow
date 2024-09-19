//! A DataFlow Analyise and Codegen Compiler written in Rust.
//!
//! In ThrillerFlow, we introduce a nested multi-dimendional dataflow
//! graph called Extended task Dependence Graph(ETDG), a unified intermediate
//! representation that preserves a holistic view of parallelism and
//! dependency across different control and data nested levels on the code.
//! To facilitate code analysis and the later low-level code generation,
//! an ETDG concisely encodes complex control structures and precisely
//! represents the iteration-level data dependencies with and acyclic graph.
//! For a clear exposition, ETDG borrows the concepts from the reduced dependence
//! graph used classical compilers with the Static Control Program(SCoP) modeling
//! employed in polyhedral compilers.
//!
//! In ThrillerFlow, we introduce some important data structures:
//!
//! - [`ThrillerBlock`] represents the data-parallel repetition of a
//!   dataflow task int form of a d-dimensional dataflow node.
//! - [`AttachedEdge`] is an edge that connects a source and destination buffer
//!   with additional access pattern information [`AccessMap`].
//! - [`AccessMap`] represents a multi-dimensional access pattern.
//! - [`ThrillerGraph`] represents a d-dimensional dataflow task graph within nested loops.
//! - [`Buffer`] represents an addressable instance declared in user mode, which contains
//!   [`Shape`], [`Layout`] and [`BufType`].

#![deny(warnings)]
#![deny(missing_docs)]

mod access;
mod buffer;
mod dataflow;
mod engine;
mod error;
mod id;
mod kernels;
mod log;
mod shape;
mod task;
mod var;

pub use access::{AccessMap, AccessMatrix, AccessOffset};
pub use buffer::{BufType, Buffer};
pub use dataflow::{
    AttachedEdge, ThrillerBlock, ThrillerEdge, ThrillerGraph, ThrillerNode, ThrillerNodeInner,
};
pub use engine::{BlockLayout, BlockShape, ThrillerEngine};
pub use error::{ThrillerError, ThrillerResult};
pub use log::{debug, error, info, init_logger, set_max_level, trace, warn};
pub use shape::{Dim, Dimension, Layout, Shape};
pub use task::{Gemm, Task};
pub use var::{IterationBound, IterationVar, RegularVar, Var};

use id::ID_COUNTER;

/// GPU Memory Level.
#[derive(Default, Clone, Copy, PartialEq)]
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

    init_logger();
    set_max_level("debug");
}

/// Return the next unique ID.
pub fn next_id() -> usize {
    unsafe { ID_COUNTER.get_mut().unwrap().next() }
}
