use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::{AttachedEdge, ThrillerGraph};
use crate::error::{ThrillerError, ThrillerResult};
use crate::task::Task;
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
    inputs: Vec<Rc<AttachedEdge>>,
    outputs: Vec<Rc<AttachedEdge>>,
    mem_level: MemoryLevel,
    #[allow(dead_code)]
    subgraph: Rc<ThrillerGraph>,
    block_type: BlockType,
}

impl ThrillerBlock {
    /// Create a new ThrillerBlock with the given inputs, outputs, memory level, subgraph, and block type.
    pub fn new(
        inputs: Vec<Rc<AttachedEdge>>,
        outputs: Vec<Rc<AttachedEdge>>,
        mem_level: MemoryLevel,
        subgraph: Rc<ThrillerGraph>,
        block_type: BlockType,
    ) -> Self {
        ThrillerBlock {
            inputs,
            outputs,
            mem_level,
            subgraph,
            block_type,
        }
    }

    pub(crate) fn gen_loop(&mut self) -> ThrillerResult<String> {
        let mut code = String::new();

        // Check if input edges have the same access pattern.
        let access_map = self.inputs[0].get_access().as_ref().unwrap();
        for input in &self.inputs {
            if input.get_access().as_ref().unwrap() != access_map {
                return Err(ThrillerError::InvalidAccessPattern);
            }
        }

        Ok(code)
    }

    /// Generate load code for the block inputs.
    pub(crate) fn gen_load(&self) -> String {
        let mut code = String::new();

        // Generate load inputs.
        match self.mem_level {
            MemoryLevel::Register => {
                for input in &self.inputs {
                    code.push_str(&format!(
                        "copy_2d_tile_s2r({}, {});\n",
                        input.get_src_name(),
                        input.get_dst_name()
                    ));
                }
            }

            MemoryLevel::Shared => {
                for input in &self.inputs {
                    code.push_str(&format!(
                        "copy_2d_tile_g2s({}, {});\n",
                        input.get_src_name(),
                        input.get_dst_name()
                    ));
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
                        code.push_str(&format!(
                            "copy_2d_tile_r2s({}, {});\n",
                            output.get_src_name(),
                            output.get_dst_name()
                        ));
                    }
                }

                MemoryLevel::Shared => {
                    for output in &self.outputs {
                        code.push_str(&format!(
                            "copy_2d_tile_s2g({}, {});\n",
                            output.get_src_name(),
                            output.get_dst_name()
                        ));
                    }
                }

                _ => {}
            },

            BlockType::Reduce => {}
        }
        code
    }

    // #[allow(dead_code)]
    // pub(crate) fn reduce(&self) -> Option<&Vec<Rc<Buffer>>> {
    //     match self.block_type {
    //         BlockType::Reduce => Some(&self.outputs),
    //         _ => None,
    //     }
    // }

    #[allow(dead_code)]
    pub(crate) fn get_mem_level(&self) -> MemoryLevel {
        self.mem_level
    }
}

impl Task for ThrillerBlock {
    fn emit(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        code += "{\n";
        code += &self.gen_load();
        code += self.subgraph.emit()?.as_str();
        code += &self.gen_store();
        code += "}\n";
        Ok(code)
    }
}
