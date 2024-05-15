/// Memory management.
pub struct Memory;

impl Memory {
    /// Emit a shared buffer declaration.
    pub fn emit_shared_buf_decl() -> String {
        let mut code = String::new();
        code += "extern __shared__ __align__(sizeof(double)) unsigned char shared_buf[];\n";
        code += "auto* shm = reinterpret_cast<Element*>(shared_buf);\n";

        code
    }
}
