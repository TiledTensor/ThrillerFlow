use crate::ThrillerResult;

mod compute;
mod copy;

pub use compute::Gemm;

// pub struct Context<'a> {
//     input_edges: &'a [ThrillerEdge],
//     output_edges: &'a [ThrillerEdge],
// }

/// A trait to represent a task.
pub trait Task {
    /// Emit the task into SIMT code.
    fn emit(&self) -> ThrillerResult<String>;
}
