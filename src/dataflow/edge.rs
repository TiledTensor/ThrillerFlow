use std::rc::Rc;
use std::vec::Vec;

use crate::access::AccessMap;
use crate::dataflow::ThrillerNode;
use crate::task::Task;

/// `ThrillerEdge` repersent load/store in dataflow graph.
#[allow(dead_code)]
pub struct ThrillerEdge {
    in_nodes: Vec<Rc<ThrillerNode>>,
    out_nodes: Vec<Rc<ThrillerNode>>,
    out_block: Option<Rc<ThrillerNode>>,
    access: AccessMap,
    task: Box<dyn Task>,
}

impl ThrillerEdge {
    /// Create a new empty `ThrillerEdge`.
    pub fn new(access: AccessMap, task: Box<dyn Task>) -> Self {
        ThrillerEdge {
            in_nodes: Vec::new(),
            out_nodes: Vec::new(),
            out_block: None,
            access,
            task,
        }
    }
}
