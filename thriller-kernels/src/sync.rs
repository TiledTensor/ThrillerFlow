/// Synchronization primitives.
pub struct Sync;

impl Sync {
    /// Emit a synchronization primitive.
    pub fn emit_sync() -> String {
        "__syncthreads();\n".to_string()
    }

    /// Emit a copy async primitive.
    pub fn emit_copy_async() -> String {
        "__copy_async();\n".to_string()
    }
}
