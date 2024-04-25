mod copy;
pub trait Task {
    /// Emit the task into SIMT code.
    fn emit(&self) -> String;
}
