use crate::ThrillerResult;

mod compute;
mod copy;

pub use compute::Gemm;

/// A trait to represent a task.
pub trait Task {
    /// Emit the task into SIMT code.
    fn emit(&self) -> ThrillerResult<String>;
}
