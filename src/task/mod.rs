use crate::ThrillerResult;

mod compute;
mod copy;

/// A trait to represent a task.
pub trait Task {
    /// Emit the task into SIMT code.
    fn emit(&self) -> ThrillerResult<String>;
}
