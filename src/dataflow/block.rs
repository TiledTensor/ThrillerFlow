use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::{AttachedEdge, ThrillerGraph};
use crate::error::{ThrillerError, ThrillerResult};
use crate::task::Task;
use crate::var::Var;
use crate::{next_id, AccessMap, MemoryLevel};

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
    id: usize,
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
            id: next_id(),
        }
    }

    pub(crate) fn gen_loop_load(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        for edge in self.inputs.iter() {
            if let Some(access) = edge.get_access() {
                // TODO: Add access pattern support for load operation.
                let load = |access_map: &AccessMap| -> ThrillerResult<String> {
                    self.gen_load(access_map, edge)
                };

                code += access.gen_loop_access(load)?.as_str();
            }
        }
        Ok(code)
    }

    /// Generate load code for the block inputs.
    pub(crate) fn gen_load(
        &self,
        _access_map: &AccessMap,
        edge: &Rc<AttachedEdge>,
    ) -> ThrillerResult<String> {
        // TODO: This is not a final version of the load code generation. It is just a pseudocode representation of the formalized data flow.
        let mut code = String::new();
        // Generate load inputs.

        match self.mem_level {
            MemoryLevel::Register => {
                let access_map = edge
                    .get_access()
                    .as_ref()
                    .ok_or(ThrillerError::MissingAccessMap)?;

                let loop_depth = access_map.get_loop_depth();
                if loop_depth != 1 {
                    return Err(ThrillerError::InvalidLoadAccess);
                }

                let offsets = access_map.get_access_offsets();
                let matrixs = access_map.get_access_matrixs();

                let iter_vars = access_map.get_iter_vars();

                code.push_str(&format!(
                        "copy_2d_tile_s2r({src}[{src_access} * {src_index} + {src_offset}], {dst}[{dst_access} * {dst_index} + {dst_offset}]);\n",
                        src = edge.get_src_name(),
                        src_access = matrixs[0].0[0][0],
                        src_offset = offsets[0].0[0],
                        src_index = iter_vars[0].get_name(),
                        dst = edge.get_dst_name(),
                        dst_index = iter_vars[0].get_name(),
                        dst_offset = offsets[1].0[0],
                        dst_access = matrixs[1].0[0][0]
                    ));
            }

            MemoryLevel::Shared => {
                code.push_str(&format!(
                    "copy_2d_tile_g2s({}, {});\n",
                    edge.get_src_name(),
                    edge.get_dst_name()
                ));
            }

            _ => {}
        }
        Ok(code)
    }

    /// Generate store code for the block outputs.
    pub(crate) fn gen_store(&self) -> ThrillerResult<String> {
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
        Ok(code)
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

    pub(crate) fn emit_block(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        code += &self.gen_loop_load()?;
        code += self.subgraph.emit()?.as_str();
        code += &self.gen_store()?;
        Ok(code)
    }
}

impl Task for ThrillerBlock {
    fn emit(&self) -> ThrillerResult<String> {
        self.emit_block()
    }

    fn get_name(&self) -> String {
        format!("block_{}", self.id)
    }
}
