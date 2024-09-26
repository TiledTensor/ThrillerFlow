mod gemm;
mod map;

#[allow(dead_code)]
/// Compute Type.
pub enum Compute {
    Gemm,
    Axpby,
}

pub use gemm::Gemm;
pub use map::Convert;
