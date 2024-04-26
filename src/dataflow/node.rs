use std::rc::Rc;

use crate::dataflow::{ThrillerEdge, ThrillerGraph};
use crate::task::Task;

/// `ThrillerNodeInnrer` is an enum to represent either an operation or a block.
#[allow(dead_code)]
pub enum ThrillerNodeInner {
    Op(Box<dyn Task>),
    Block(Rc<ThrillerGraph>),
}

/// A Thriller Dataflow Node that represents either a block of subgraph or an operation.
#[allow(dead_code)]
pub struct ThrillerNode {
    inner: Box<ThrillerNodeInner>,
    in_edges: Vec<Rc<ThrillerEdge>>,
    out_edges: Vec<Rc<ThrillerEdge>>,
}

impl ThrillerNode {
    /// Create a new `ThrillerNode` with the given inner type.
    pub fn new(inner: ThrillerNodeInner) -> Self {
        ThrillerNode {
            inner: Box::new(inner),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
        }
    }
}
