use super::GraphPass;
use crate::{dataflow::ThrillerGraph, BufType, ThrillerNodeInner};

/// AllocateVar
pub struct AllocateVar {
    code: String,
}

impl AllocateVar {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    #[doc(hidden)]
    pub fn code(&self) -> String {
        self.code.clone()
    }
}

impl GraphPass for AllocateVar {
    fn run(&mut self, graph: &mut ThrillerGraph) {
        // Transver the graph and allocate variables.
        for node in &graph.nodes {
            let node = node.borrow();
            let inner = node.get_inner();
            match inner {
                ThrillerNodeInner::Buffer(buf) => {
                    let btype = buf.get_typing();
                    match btype {
                        BufType::GlobalTile => {
                            self.code +=
                                format!("Global{} {};\n", buf.get_name(), buf.get_name()).as_str();
                        }

                        &BufType::SharedTile => {
                            self.code +=
                                format!("Shared{} {};\n", buf.get_name(), buf.get_name()).as_str();
                        }

                        &BufType::RegTile | &BufType::RegVec => {
                            self.code +=
                                format!("Reg{} {};\n", buf.get_name(), buf.get_name()).as_str();
                        }
                    }
                }
                ThrillerNodeInner::Block(block) => {
                    // Catch the graph in the block.
                    let mut sub_graph = block.subgraph.borrow_mut();
                    // Recursively allocate variables in the block.
                    self.run(&mut sub_graph);
                }
                _ => {}
            }
        }
    }
}
