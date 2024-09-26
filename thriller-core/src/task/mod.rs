use crate::ThrillerResult;

mod compute;
mod copy;

pub use compute::{Convert, Gemm};

/// A trait to represent a task.
pub trait Task {
    /// Emit the task into SIMT code.
    fn emit(&self) -> ThrillerResult<String>;

    /// Get the name of the task.
    fn get_name(&self) -> String;
}
