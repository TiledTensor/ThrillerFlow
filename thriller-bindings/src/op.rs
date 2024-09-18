use pyo3::prelude::*;
use thriller_core::{AccessMap, Gemm, Task};

use crate::graph::PyNode;

use std::rc::Rc;

#[pyclass(unsendable, module = "operators", name = "Gemm")]
pub struct PyGemm(pub Gemm);

#[pymethods]
impl PyGemm {
    #[new]
    fn new(a: PyRef<PyNode>, b: PyRef<PyNode>, c: PyRef<PyNode>) -> Self {
        let access_map = AccessMap::new(0, vec![]);

        let node_a = Rc::clone(&a.0);
        let node_b = Rc::clone(&b.0);
        let node_c = Rc::clone(&c.0);

        let gemm = Gemm::new(vec![node_a, node_b], node_c, Rc::new(access_map));

        PyGemm(gemm)
    }

    fn codegen(&self) -> PyResult<String> {
        self.0
            .emit()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))
    }
}
