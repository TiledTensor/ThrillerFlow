use pyo3::prelude::*;
use thriller_core::{BufType, Buffer, Dim, Layout};

#[pyclass]
pub struct PyBuffer(pub Buffer);

#[pyclass]
pub struct PyLayout(pub Layout<Dim>);

#[pyclass]
pub struct PyBufType(pub BufType);

#[pymethods]
impl PyBuffer {
    #[new]
    fn new(name: String, dim: Vec<usize>, layout: &PyLayout, buf_type: &PyBufType) -> Self {
        Self(Buffer::new(
            name.as_str(),
            buf_type.0,
            &dim,
            layout.0.clone(),
        ))
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
