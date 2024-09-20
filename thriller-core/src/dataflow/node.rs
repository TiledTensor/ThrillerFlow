use std::cell::RefCell;
use std::rc::Rc;

use crate::dataflow::{ThrillerBlock, ThrillerEdge};
use crate::task::Task;
use crate::{next_id, Buffer, ThrillerResult};

/// [`ThrillerNodeInner`] is an enum to represent either an operation or a block.
///
/// In [`ThrillerNode`], [`Buffer`] Node will merely be a placeholder that represents
/// tensor in memory, without actually performing any computation.
///
/// [`Task`] represents an abstract operation that can be executed by a [`crate::ThrillerGraph`].
///
/// [`ThrillerBlock`] represents a data-parallel repetition of a dataflow task in the form of a
/// loop.
pub enum ThrillerNodeInner {
    /// An operation.
    Op(Box<dyn Task>),
    /// A buffer.
    Buffer(Rc<Buffer>),
    /// A subgraph block.
    Block(Rc<ThrillerBlock>),
}

/// [`ThrillerNode`] represents an abstract node element that can represent a Buffer Node,
/// Operator Node and Block Node.
pub struct ThrillerNode {
    inner: Box<ThrillerNodeInner>,
    in_edges: Vec<Rc<ThrillerEdge>>,
    out_edges: Vec<Rc<ThrillerEdge>>,
    prevs: Vec<Rc<RefCell<ThrillerNode>>>,
    nexts: Vec<Rc<RefCell<ThrillerNode>>>,
    id: usize,
    in_degrees: usize,
}

impl ThrillerNode {
    /// Create a new `ThrillerNode` with the given inner type.
    pub fn new(inner: ThrillerNodeInner) -> Self {
        ThrillerNode {
            inner: Box::new(inner),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
            prevs: Vec::new(),
            nexts: Vec::new(),
            id: next_id(),
            in_degrees: 0,
        }
    }

    /// Get the name of the node.
    pub fn get_node_name(&self) -> String {
        match self.inner.as_ref() {
            ThrillerNodeInner::Buffer(buffer) => buffer.get_name().clone(),
            ThrillerNodeInner::Op(task) => task.get_name().clone(),
            ThrillerNodeInner::Block(block) => block.get_name().clone(),
        }
    }

    pub(crate) fn get_id(&self) -> usize {
        self.id
    }

    pub(crate) fn get_in_degrees(&self) -> usize {
        self.in_degrees
    }

    #[allow(dead_code)]
    pub(crate) fn get_prevs(&self) -> &Vec<Rc<RefCell<ThrillerNode>>> {
        &self.prevs
    }

    pub(crate) fn get_nexts(&self) -> &Vec<Rc<RefCell<ThrillerNode>>> {
        &self.nexts
    }

    #[doc(hidden)]
    pub fn get_inner(&self) -> &ThrillerNodeInner {
        &self.inner
    }

    pub(crate) fn add_in_edge(&mut self, edge: Rc<ThrillerEdge>) {
        self.in_edges.push(edge);
    }

    pub(crate) fn add_out_edge(&mut self, edge: Rc<ThrillerEdge>) {
        self.out_edges.push(edge);
    }

    pub(crate) fn add_prev(&mut self, node: Rc<RefCell<ThrillerNode>>) {
        self.prevs.push(node);
    }

    pub(crate) fn add_next(&mut self, node: Rc<RefCell<ThrillerNode>>) {
        self.nexts.push(node);
    }

    pub(crate) fn inc_in_degrees(&mut self) {
        self.in_degrees += 1;
    }
}

impl Task for ThrillerNode {
    fn emit(&self) -> ThrillerResult<String> {
        match self.inner.as_ref() {
            ThrillerNodeInner::Op(task) => task.emit(),
            _ => panic!("Node is not an operation"),
        }
    }

    fn get_name(&self) -> String {
        self.get_node_name()
    }
}
