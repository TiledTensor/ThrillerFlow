mod copy;

/// A trait to represent a task in the dataflow graph.
pub trait Task {
    /// Emit the task into SIMT code.
    fn emit(&self) -> String;
}
