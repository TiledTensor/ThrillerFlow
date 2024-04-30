use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::ThrillerGraph;
use crate::task::Task;
use crate::Buffer;
use crate::MemoryLevel;

/// A map relation from inputs into outputs.
pub enum BlockType {
    /// Map: one to one
    Map,
    /// Reduce: many to one
    Reduce,
}

/// A Thriller Dataflow Block representing a memory level subgraph.
pub struct ThrillerBlock {
    inputs: Vec<Rc<Buffer>>,
    outputs: Vec<Rc<Buffer>>,
    mem_level: MemoryLevel,
    subgraph: Rc<ThrillerGraph>,
    block_type: BlockType,
}

impl ThrillerBlock {
    /// Generate load code for the block inputs.
    pub(crate) fn gen_load(&self) -> String {
        let mut code = String::new();

        // Generate load inputs.
        match self.mem_level {
            MemoryLevel::Register => {
                for input in &self.inputs {
                    code.push_str(&format!("copy_2d_tile_s2r({});\n", input.get_name()));
                }
            }

            MemoryLevel::Shared => {
                for input in &self.inputs {
                    code.push_str(&format!("copy_2d_tile_g2s({});\n", input.get_name()));
                }
            }

            _ => {}
        }
        code
    }

    /// Generate store code for the block outputs.
    pub(crate) fn gen_store(&self) -> String {
        let mut code = String::new();

        // Generate store outputs.
        match self.block_type {
            BlockType::Map => match self.mem_level {
                MemoryLevel::Register => {
                    for output in &self.outputs {
                        code.push_str(&format!("copy_2d_tile_r2s({});\n", output.get_name()));
                    }
                }

                MemoryLevel::Shared => {
                    for output in &self.outputs {
                        code.push_str(&format!("copy_2d_tile_s2g({});\n", output.get_name()));
                    }
                }

                _ => {}
            },

            BlockType::Reduce => {}
        }
        code
    }

    pub(crate) fn reduce(&self) -> Option<&Vec<Rc<Buffer>>> {
        match self.block_type {
            BlockType::Reduce => Some(&self.outputs),
            _ => None,
        }
    }
}

impl Task for ThrillerBlock {
    fn emit(&self) -> String {
        let mut code = String::new();

        code += &self.gen_load();
        code += self.subgraph.emit().as_str();
        code += &self.gen_store();

        code
    }
}
