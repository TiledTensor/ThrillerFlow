use pyo3::prelude::*;
use thriller_core::Buffer;

#[pyclass]
pub struct PyBuffer(pub Buffer);

#[pymethods]
impl PyBuffer {
    #[new]
    fn new(name: String) -> Self {
        Self(Buffer::new(name.as_str()))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("name: {}", self.0.get_name()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "id: {}, name: {}",
            self.0.get_id(),
            self.0.get_name()
        ))
    }
}
