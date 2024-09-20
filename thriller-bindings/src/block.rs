use std::rc::Rc;

use thriller_core::{AttachedEdge, Task, ThrillerBlock};

use pyo3::{prelude::*, types::PyList};

use crate::{access::PyAccessMap, buffer::PyBuffer, graph::PyGraph, var::PyIterationVar};

#[pyclass(unsendable, module = "block", name = "Block")]
pub struct PyBlock(pub Rc<ThrillerBlock>);

#[pyclass(unsendable, module = "block", name = "AttachedEdge")]
pub struct PyAttachedEdge(pub Rc<AttachedEdge>);

#[pymethods]
impl PyBlock {
    #[new]
    fn new(
        inputs: &Bound<PyList>,
        outputs: &Bound<PyList>,
        subgraph: PyRef<PyGraph>,
        ivars: &Bound<PyList>,
    ) -> PyResult<Self> {
        let inputs = inputs
            .into_iter()
            .map(|edge| {
                // TODO(KuangjuX): fix `unwarp()`.
                let edge = edge.extract::<PyRef<PyAttachedEdge>>().unwrap();
                Rc::clone(&edge.0)
            })
            .collect::<Vec<_>>();

        let outputs = outputs
            .into_iter()
            .map(|edge| {
                // TODO(KuangjuX): fix `unwarp()`.
                let edge = edge.extract::<PyRef<PyAttachedEdge>>().unwrap();
                Rc::clone(&edge.0)
            })
            .collect::<Vec<_>>();

        let ivars = ivars
            .into_iter()
            .map(|ivar| {
                // TODO(KuangjuX): fix `unwarp()`.
                let ivar = ivar.extract::<PyRef<PyIterationVar>>().unwrap();
                Rc::clone(&ivar.0)
            })
            .collect::<Vec<_>>();

        let subgraph = Rc::clone(&subgraph.0);

        let block = ThrillerBlock::new(inputs, outputs, subgraph, ivars);

        Ok(PyBlock(Rc::new(block)))
    }

    fn codegen(&self) -> PyResult<String> {
        self.0
            .emit()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }
}

#[pymethods]
impl PyAttachedEdge {
    #[new]
    fn new(src: PyRef<PyBuffer>, dst: PyRef<PyBuffer>, map: PyRef<PyAccessMap>) -> Self {
        let src = Rc::clone(&src.0);
        let dst = Rc::clone(&dst.0);
        let map = Rc::clone(&map.0);
        PyAttachedEdge(Rc::new(AttachedEdge::new(src, dst, map)))
    }
}
