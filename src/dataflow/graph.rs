use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::{ThrillerEdge, ThrillerNode};
use crate::task::Task;
use crate::MemoryLevel;

/// Thriller Dataflow Graph structure.
#[allow(dead_code)]
#[derive(Default)]
pub struct ThrillerGraph {
    id: usize,
    nodes: Vec<Rc<ThrillerNode>>,
    in_edges: Vec<Rc<ThrillerEdge>>,
    out_edges: Vec<Rc<ThrillerEdge>>,
    intra_edges: Vec<Rc<ThrillerEdge>>,
    mem_level: MemoryLevel,
}

impl ThrillerGraph {
    /// Create a new empty ThrillerGraph.
    pub fn new(mem_level: MemoryLevel) -> Self {
        ThrillerGraph {
            id: unsafe { crate::id::ID_COUNTER.get_mut().unwrap().next() },
            nodes: Vec::new(),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
            intra_edges: Vec::new(),
            mem_level,
        }
    }

    /// Topological sort the nodes in the graph.
    pub fn topo_sort(&self) -> Vec<Rc<ThrillerNode>> {
        let mut sorted_nodes = Vec::new();
        // (id, (in_degrees, node))
        let mut in_degrees: HashMap<usize, (usize, &Rc<ThrillerNode>)> = HashMap::new();

        for node in &self.nodes {
            in_degrees.insert(node.get_id(), (node.get_in_degrees(), node));
        }

        while !in_degrees.is_empty() {
            let node_ids = in_degrees.keys().cloned().collect::<Vec<_>>();
            for node_id in node_ids {
                let (in_degree, node) = in_degrees[&node_id];
                if in_degree == 0 {
                    sorted_nodes.push(node.clone());

                    for successor in node.get_successors() {
                        let successor_id = successor.get_id();
                        let (in_degree, _) = in_degrees.get_mut(&successor_id).unwrap();
                        *in_degree -= 1;
                    }

                    in_degrees.remove(&node_id);
                }
            }
        }

        sorted_nodes
    }
}

impl Task for ThrillerGraph {
    fn emit(&self) -> String {
        todo!()
    }
}
