use std::vec::Vec;

use crate::dataflow::node::ThrillerNode;
use crate::task::Task;

/// Thriller Dataflow Graph structure.
#[allow(dead_code)]
#[derive(Default)]
pub struct ThrillerGraph {
    id: usize,
    nodes: Vec<ThrillerNode>,
}

impl ThrillerGraph {
    /// Create a new empty ThrillerGraph.
    pub fn new() -> Self {
        ThrillerGraph {
            id: unsafe { crate::id::ID_COUNTER.get_mut().unwrap().next() },
            nodes: Vec::new(),
        }
    }

    /// Topological sort the nodes in the graph.
    pub fn topo_sort(&self) -> Vec<&ThrillerNode> {
        todo!()
    }
}

impl Task for ThrillerGraph {
    fn emit(&self) -> String {
        todo!()
    }
}
