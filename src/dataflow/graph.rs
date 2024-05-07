use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::{ThrillerEdge, ThrillerNode, ThrillerNodeInner};
use crate::task::Task;
use crate::{next_id, MemoryLevel, ThrillerResult};

/// Thriller Dataflow Graph structure.
#[allow(dead_code)]
#[derive(Default)]
pub struct ThrillerGraph {
    id: usize,
    nodes: Vec<Rc<ThrillerNode>>,
    in_edges: Vec<Rc<ThrillerEdge>>,
    out_edges: Vec<Rc<ThrillerEdge>>,
    intra_edges: Vec<Rc<ThrillerEdge>>,
    sorted_nodes: Option<Vec<Rc<ThrillerNode>>>,
    mem_level: MemoryLevel,
}

impl ThrillerGraph {
    /// Create a new empty ThrillerGraph.
    pub fn new(mem_level: MemoryLevel) -> Self {
        ThrillerGraph {
            id: next_id(),
            nodes: Vec::new(),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
            intra_edges: Vec::new(),
            sorted_nodes: None,
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

        // self.sorted_nodes = Some(sorted_nodes);

        // self.sorted_nodes.as_ref().unwrap()
        sorted_nodes
    }

    /// Get the sorted nodes of the graph.
    pub fn get_sorted_nodes(&self) -> &Vec<Rc<ThrillerNode>> {
        self.sorted_nodes.as_ref().unwrap()
    }
}

impl Task for ThrillerGraph {
    fn emit(&self) -> ThrillerResult<String> {
        #[allow(unused_mut)]
        let mut code = String::new();
        // let sorted_nodes = if let Some(sorted_nodes) = &self.sorted_nodes {
        //     sorted_nodes
        // } else {
        //     self.topo_sort()
        // };

        let sorted_nodes = self.topo_sort();

        let mut compute_nodes = Vec::new();
        let mut block_nodes = Vec::new();
        let mut buffer_nodes = Vec::new();

        let mut block_codes = Vec::new();
        let mut compute_codes = Vec::new();

        for node in sorted_nodes {
            match node.get_inner() {
                ThrillerNodeInner::Op(_) => compute_nodes.push(node),
                ThrillerNodeInner::Block(_) => block_nodes.push(node),
                ThrillerNodeInner::Buffer(_) => buffer_nodes.push(node),
            }
        }

        for node in block_nodes {
            let block = if let ThrillerNodeInner::Block(block) = node.get_inner() {
                block
            } else {
                unreachable!()
            };
            let mut block_code = block.emit()?;
            block_code.push_str("__syncthreads();\n");

            block_codes.push(block_code);
        }

        for node in compute_nodes {
            let compute = if let ThrillerNodeInner::Op(compute) = node.get_inner() {
                compute
            } else {
                unreachable!()
            };

            compute_codes.push(compute.emit());
        }

        Ok(code)
    }
}
