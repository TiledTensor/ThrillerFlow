mod gemm;

#[allow(dead_code)]
/// Compute Type.
pub enum Compute {
    Gemm,
    Axpby,
}

pub use gemm::Gemm;
