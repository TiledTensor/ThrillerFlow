#[allow(dead_code)]
/// Copy Primitive.
pub struct Copy;

impl Copy {
    #[allow(dead_code)]
    /// Emit a copy from a global buffer to a shared buffer.
    pub fn emit_copy_2d_tile_g2s<T: AsRef<str>>(
        src: T,
        src_index: usize,
        dst: T,
        dst_index: usize,
    ) -> String {
        format!(
            "copy_2d_tile_g2s({src}[{src_index}], {dst}[{dst_index}])",
            src = src.as_ref(),
            src_index = src_index,
            dst = dst.as_ref(),
            dst_index = dst_index
        )
    }
}
