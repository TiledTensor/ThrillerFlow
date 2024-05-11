use std::cell::RefCell;
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
    nodes: Vec<Rc<RefCell<ThrillerNode>>>,
    edges: Vec<Rc<ThrillerEdge>>,
    sorted_nodes: Option<Vec<Rc<ThrillerNode>>>,
    mem_level: MemoryLevel,
}

impl ThrillerGraph {
    /// Create a new empty ThrillerGraph.
    pub fn new(mem_level: MemoryLevel) -> Self {
        ThrillerGraph {
            id: next_id(),
            nodes: Vec::new(),
            edges: Vec::new(),
            sorted_nodes: None,
            mem_level,
        }
    }

    /// Add nodes into the graph.
    pub fn add_nodes(&mut self, nodes: Vec<Rc<RefCell<ThrillerNode>>>) {
        self.nodes.extend(nodes);
    }

    /// Add edges into the graph.
    pub fn add_edges(&mut self, edges: Vec<Rc<ThrillerEdge>>) {
        self.edges.extend(edges);
    }

    /// Connect the nodes in the graph.
    pub fn connect(&mut self) {
        for edge in &self.edges {
            let src = edge.get_src();
            let dst = edge.get_dst();

            let mut src_ref = src.borrow_mut();
            let mut dst_ref = dst.borrow_mut();

            src_ref.add_out_edge(edge.clone());
            dst_ref.add_in_edge(edge.clone());

            src_ref.add_next(dst.clone());
            dst_ref.add_prev(src.clone());

            dst_ref.inc_in_degrees();
        }
    }

    /// Topological sort the nodes in the graph.
    pub fn topo_sort(&self) -> Vec<Rc<RefCell<ThrillerNode>>> {
        let mut sorted_nodes = Vec::new();
        // (id, (in_degrees, node))
        let mut in_degrees: HashMap<usize, (usize, &Rc<RefCell<ThrillerNode>>)> = HashMap::new();

        for node in &self.nodes {
            let ref_node = node.borrow();
            in_degrees.insert(ref_node.get_id(), (ref_node.get_in_degrees(), node));
            println!(
                "{} have {} in_degrees.",
                ref_node.get_id(),
                ref_node.get_in_degrees()
            );
        }

        while !in_degrees.is_empty() {
            let node_ids = in_degrees.keys().cloned().collect::<Vec<_>>();
            for node_id in node_ids {
                let (in_degree, node) = in_degrees[&node_id];
                if in_degree == 0 {
                    sorted_nodes.push(node.clone());

                    for next in node.borrow_mut().get_nexts() {
                        let next_id = next.borrow().get_id();
                        let (in_degree, _) = in_degrees.get_mut(&next_id).unwrap();
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
        let mut code = String::new();
        // let sorted_nodes = if let Some(sorted_nodes) = &self.sorted_nodes {
        //     sorted_nodes
        // } else {
        //     self.topo_sort()
        // };

        let sorted_nodes = self.topo_sort();

        // let mut conpute_nodes = Vec::new();

        for node in sorted_nodes {
            // match node.borrow().get_inner() {
            //     ThrillerNodeInner::Op(op) => code += op.emit()?.as_str(),
            //     _ => {}
            // }

            if let ThrillerNodeInner::Op(op) = node.borrow().get_inner() {
                code += op.emit()?.as_str();
            }
        }

        // let sorted_nodes = self.topo_sort();

        // let mut compute_nodes = Vec::new();
        // let mut block_nodes = Vec::new();
        // let mut buffer_nodes = Vec::new();

        // let mut block_codes = Vec::new();
        // let mut compute_codes = Vec::new();

        // for node in sorted_nodes {
        //     match node.borrow().get_inner() {
        //         ThrillerNodeInner::Op(_) => compute_nodes.push(node),
        //         ThrillerNodeInner::Block(_) => block_nodes.push(node),
        //         ThrillerNodeInner::Buffer(_) => buffer_nodes.push(node),
        //     }
        // }

        // for node in block_nodes {
        //     let block = if let ThrillerNodeInner::Block(block) = node.borrow().get_inner() {
        //         block
        //     } else {
        //         unreachable!()
        //     };
        //     let mut block_code = block.emit()?;
        //     block_code.push_str("__syncthreads();\n");

        //     block_codes.push(block_code);
        // }

        // for node in compute_nodes {
        //     let compute = if let ThrillerNodeInner::Op(compute) = node.borrow().get_inner() {
        //         compute
        //     } else {
        //         unreachable!()
        //     };

        //     compute_codes.push(compute.emit());
        // }

        Ok(code)
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
