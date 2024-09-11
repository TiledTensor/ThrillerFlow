use std::rc::Rc;

use pyo3::prelude::*;
use thriller_core::{BufType, Buffer, Dim, Layout};

#[pyclass(unsendable)]
pub struct PyBuffer(pub Rc<Buffer>);

#[pyclass]
pub enum PyLayout {
    RowMajor,
    ColMajor,
}

#[pyclass]
pub enum PyBufType {
    GlobalTile,
    SharedTile,
    RegTile,
    RegVec,
}

#[pymethods]
impl PyBuffer {
    #[new]
    fn new(name: String, dim: Vec<usize>, py_layout: &PyLayout, py_buf_type: &PyBufType) -> Self {
        let layout: Layout<Dim> = match py_layout {
            PyLayout::RowMajor => Layout::RowMajor,
            PyLayout::ColMajor => Layout::ColumnMajor,
        };

        let buf_type = match py_buf_type {
            PyBufType::GlobalTile => BufType::GlobalTile,
            PyBufType::SharedTile => BufType::SharedTile,
            PyBufType::RegTile => BufType::RegTile,
            PyBufType::RegVec => BufType::RegVec,
        };

        Self(Rc::new(Buffer::new(name.as_str(), buf_type, &dim, layout)))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "[name: {}, shape: {:?}, type: {:?}]",
            self.0.get_name(),
            self.0.get_shape(),
            self.0.get_typing()
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "[id: {}, name: {}, shape: {:?}, type: {:?}]",
            self.0.get_id(),
            self.0.get_name(),
            self.0.get_shape(),
            self.0.get_typing()
        ))
    }
}
