use pyo3::prelude::*;
use pyo3::types::PyList;

use thriller_core::{
    AccessMap, AllocateVar, Convert, DataType, Gemm, GraphPass, Task, ThrillerEdge, ThrillerGraph,
    ThrillerNode, ThrillerNodeInner,
};

use crate::buffer::PyBuffer;
use crate::{block::PyBlock, dtype::PyDType};

use std::{cell::RefCell, rc::Rc};

#[pyclass(unsendable, module = "graph", name = "Graph")]
pub struct PyGraph(pub Rc<RefCell<ThrillerGraph>>);

#[pymethods]
impl PyGraph {
    #[new]
    fn empty() -> PyGraph {
        PyGraph(Rc::new(RefCell::new(ThrillerGraph::new())))
    }

    fn add_nodes(&mut self, nodes: &Bound<'_, PyList>) -> PyResult<()> {
        let nodes = nodes
            .into_iter()
            .map(|node| {
                // TODO(KuangjuX): fix `unwarp`.
                let node = node.extract::<PyRef<PyNode>>().unwrap();
                Rc::clone(&node.0)
            })
            .collect::<Vec<_>>();

        self.0.borrow_mut().add_nodes(nodes);
        Ok(())
    }

    fn add_edges(&mut self, edges: &Bound<'_, PyList>) -> PyResult<()> {
        let edges = edges
            .into_iter()
            .map(|edge| {
                // TODO(KuangjuX): fix `unwarp`.
                let edge = edge.extract::<PyRef<PyEdge>>().unwrap();
                Rc::clone(&edge.0)
            })
            .collect::<Vec<_>>();

        self.0.borrow_mut().add_edges(edges);
        Ok(())
    }

    fn connect(&mut self) {
        self.0.borrow_mut().connect();
    }

    fn allocate_var(&mut self) -> PyResult<String> {
        let mut graph = self.0.borrow_mut();
        let mut pass = AllocateVar::new();
        pass.run(&mut graph);
        Ok(pass.code().clone())
    }

    fn codegen(&self) -> PyResult<String> {
        self.0
            .borrow()
            .emit()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }
}

#[pyclass(unsendable, module = "graph", name = "Node")]
pub struct PyNode(pub Rc<RefCell<ThrillerNode>>);

#[pymethods]
impl PyNode {
    #[staticmethod]
    fn tensor(buf: PyRef<PyBuffer>) -> Self {
        let node = ThrillerNode::new(thriller_core::ThrillerNodeInner::Buffer(Rc::clone(&buf.0)));
        PyNode(Rc::new(RefCell::new(node)))
    }

    #[staticmethod]
    fn block(block: PyRef<PyBlock>) -> Self {
        let node = ThrillerNode::new(ThrillerNodeInner::Block(Rc::clone(&block.0)));
        PyNode(Rc::new(RefCell::new(node)))
    }

    #[staticmethod]
    fn gemm(a: PyRef<PyNode>, b: PyRef<PyNode>, c: PyRef<PyNode>) -> Self {
        let access_map = AccessMap::new(0, vec![]);

        let node_a = Rc::clone(&a.0);
        let node_b = Rc::clone(&b.0);
        let node_c = Rc::clone(&c.0);

        let gemm = Gemm::new(vec![node_a, node_b], node_c, Rc::new(access_map));

        let node = ThrillerNode::new(ThrillerNodeInner::Op(Box::new(gemm)));

        PyNode(Rc::new(RefCell::new(node)))
    }

    #[staticmethod]
    fn cast(
        src: PyRef<PyBuffer>,
        dst: PyRef<PyBuffer>,
        sdtype: PyRef<PyDType>,
        ddtype: PyRef<PyDType>,
    ) -> Self {
        let sdtype = match *sdtype {
            PyDType::F32 => DataType::Float32,
            PyDType::F64 => DataType::Float64,
            PyDType::Half => DataType::Half,
            PyDType::CutlassHalf => DataType::Cutlasshalf,
        };

        let ddtype = match *ddtype {
            PyDType::F32 => DataType::Float32,
            PyDType::F64 => DataType::Float64,
            PyDType::Half => DataType::Half,
            PyDType::CutlassHalf => DataType::Cutlasshalf,
        };

        let sbuf = Rc::clone(&src.0);
        let dbuf = Rc::clone(&dst.0);

        let cast = Convert::new(sbuf, dbuf, sdtype, ddtype);
        let node = ThrillerNode::new(ThrillerNodeInner::Op(Box::new(cast)));

        PyNode(Rc::new(RefCell::new(node)))
    }

    fn codegen(&self) -> PyResult<String> {
        let node = self.0.borrow();
        node.emit()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }
}

#[pyclass(unsendable, module = "graph", name = "Edge")]
pub struct PyEdge(pub Rc<ThrillerEdge>);

#[pymethods]
impl PyEdge {
    #[new]
    fn new(src: PyRef<PyNode>, dst: PyRef<PyNode>) -> Self {
        let src = Rc::clone(&src.0);
        let dst = Rc::clone(&dst.0);
        let edge = ThrillerEdge::new(src, dst);
        PyEdge(Rc::new(edge))
    }
}
