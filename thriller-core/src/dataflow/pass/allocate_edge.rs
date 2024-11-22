use std::rc::Rc;

use super::GraphPass;
use crate::{dataflow::ThrillerGraph, AttachedEdge, BufType, ThrillerNodeInner};

/// AllocateEdge
pub struct AllocateEdge {
    code: String,
}

impl AllocateEdge {
    pub(crate) fn allocate_edge(&mut self, edge: &Rc<AttachedEdge>) {
        let in_edge_type = edge.src.get_typing();
        let in_edge_name = edge.src.get_name();
        let out_edge_type = edge.dst.get_typing();
        let out_edge_name = edge.dst.get_name();

        match (in_edge_type, out_edge_type) {
            (&BufType::GlobalTile, &BufType::SharedTile) => {
                self.code += format!("G2SLoader{}_{}", in_edge_name, out_edge_name).as_str();
            }
            _ => todo!(),
        }
    }
}

impl GraphPass for AllocateEdge {
    fn run(&mut self, graph: &mut ThrillerGraph) {
        for node in &graph.nodes {
            let node = node.borrow();
            let inner = node.get_inner();
            if let ThrillerNodeInner::Block(block) = inner {
                // Get Attched Edges
                // Load operations
                let inputs = &block.inputs;
                // Store operations
                let outputs = &block.outputs;

                // Allocate edges
                for edge in inputs {
                    self.allocate_edge(edge);
                }

                for edge in outputs {
                    self.allocate_edge(edge);
                }

                // Get subgraph
                let mut sub_graph = block.subgraph.borrow_mut();
                // Recursively allocate edges in the block.
                self.run(&mut sub_graph);
            }
        }
    }
}
