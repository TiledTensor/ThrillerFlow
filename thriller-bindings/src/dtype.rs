use pyo3::prelude::*;

#[pyclass(module = "dtype", name = "DType")]
pub enum PyDType {
    F32,
    F64,
    Half,
    CutlassHalf,
}
