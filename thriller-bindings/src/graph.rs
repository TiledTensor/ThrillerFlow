use pyo3::prelude::*;

use thriller_core::{MemoryLevel, ThrillerEdge, ThrillerGraph, ThrillerNode};

use crate::buffer::PyBuffer;

use std::{cell::RefCell, rc::Rc};

#[pyclass]
pub enum PyMemoryLevel {
    Register,
    Shared,
    Global,
}

#[pyclass(unsendable)]
pub struct PyGraph(pub ThrillerGraph);

#[pymethods]
impl PyGraph {
    #[new]
    fn new(mem_level: &PyMemoryLevel) -> PyGraph {
        let mem_level = match mem_level {
            PyMemoryLevel::Register => MemoryLevel::Register,
            PyMemoryLevel::Shared => MemoryLevel::Shared,
            PyMemoryLevel::Global => MemoryLevel::Global,
        };

        PyGraph(ThrillerGraph::new(mem_level))
    }
}

#[pyclass(unsendable)]
pub struct PyNode(pub Rc<RefCell<ThrillerNode>>);

#[pymethods]
impl PyNode {
    #[new]
    fn buffer(buf: &PyBuffer) -> Self {
        let node = ThrillerNode::new(thriller_core::ThrillerNodeInner::Buffer(Rc::new(
            buf.0.clone(),
        )));
        PyNode(Rc::new(RefCell::new(node)))
    }
}

#[pyclass(unsendable)]
pub struct PyEdge(pub ThrillerEdge);
