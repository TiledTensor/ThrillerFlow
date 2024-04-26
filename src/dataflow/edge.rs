use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::ThrillerNode;

/// `ThrillerEdge` repersent load/store in dataflow graph.
#[allow(dead_code)]
pub struct ThrillerEdge {
    in_nodes: Vec<Rc<ThrillerNode>>,
    out_nodes: Vec<Rc<ThrillerNode>>,
    out_block: Option<Rc<ThrillerNode>>,
}

impl ThrillerEdge {
    /// Create a new empty `ThrillerEdge`.
    pub fn new() -> Self {
        ThrillerEdge {
            in_nodes: Vec::new(),
            out_nodes: Vec::new(),
            out_block: None,
        }
    }
}
