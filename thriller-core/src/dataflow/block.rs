use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::{AttachedEdge, ThrillerGraph};
use crate::error::{ThrillerError, ThrillerResult};
use crate::task::Task;
use crate::var::Var;
use crate::{next_id, AccessMap, MemoryLevel};

// use thriller_kernels::Sync;
use crate::kernels::sync::Sync;

use super::loop_analysis::LoopGroup;

#[derive(PartialEq, Clone, Copy)]
/// A map relation from inputs into outputs.
pub enum BlockType {
    /// Map: one to one
    Map,
    /// Reduce: many to one
    Reduce,
}

/// A Thriller Dataflow Block representing a memory level subgraph.
pub struct ThrillerBlock {
    id: usize,
    pub(crate) inputs: Vec<Rc<AttachedEdge>>,
    pub(crate) outputs: Vec<Rc<AttachedEdge>>,
    pub(crate) mem_level: MemoryLevel,
    pub(crate) subgraph: Rc<ThrillerGraph>,
    pub(crate) block_type: BlockType,
    pub(crate) unified_access_map: Option<Rc<AccessMap>>,
    pub(crate) loop_groups: Vec<LoopGroup>,
}

impl ThrillerBlock {
    /// Create a new [`ThrillerBlock`] with the given inputs, outputs, memory level, subgraph, and block type.
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
            unified_access_map: None,
            loop_groups: vec![],
        }
    }

    /// Get the block type.
    pub fn get_block_type(&self) -> BlockType {
        self.block_type
    }

    /// Merge the same access maps into a unified access map.
    pub fn merge_access_map(&mut self) {
        // Iterate over the inputs and check if the access maps are the same.
        // If they are the same, then we can merge them into a unified access map.

        // TODO: Implement this function.
        self.inputs.windows(2).for_each(|window| {
            let (first, second) = (&window[0], &window[1]);
            assert!(
                first.get_access() == second.get_access(),
                "Access maps are not the same."
            );
        });

        self.unified_access_map = Some(self.inputs[0].get_access().as_ref().unwrap().clone());
    }

    pub(crate) fn get_inputs(&self) -> &Vec<Rc<AttachedEdge>> {
        &self.inputs
    }

    pub(crate) fn gen_loop_load(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        for edge in self.inputs.iter() {
            if let Some(access) = edge.get_access() {
                // TODO: Add access pattern support for load operation.
                // let load = |access_map: &AccessMap| -> ThrillerResult<String> {
                //     self.gen_load(access_map, edge)
                // };
                // code += access.gen_loop_access(&[load])?.as_str();
                code += self.gen_load(access, edge)?.as_str();
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
                        "copy_2d_tile_g2s({src}[{src_access} * {src_index} + {src_offset}], {dst}[{dst_access} * {dst_index} + {dst_offset}]);\n",
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

            MemoryLevel::Global => {
                unimplemented!();
            }
        }
        Ok(code)
    }

    pub(crate) fn emit_store(&self, edge: &Rc<AttachedEdge>) -> ThrillerResult<String> {
        let mut code = String::new();
        if self.block_type == BlockType::Reduce {
            return Ok(code);
        }
        // Generate store outputs.
        match self.block_type {
            BlockType::Map => match self.mem_level {
                MemoryLevel::Register => {
                    code.push_str(&format!(
                        "copy_2d_tile_r2s({}, {});\n",
                        edge.get_src_name(),
                        edge.get_dst_name()
                    ));
                }

                MemoryLevel::Shared => {
                    code.push_str(&format!(
                        "copy_2d_tile_s2g({}, {});\n",
                        edge.get_src_name(),
                        edge.get_dst_name()
                    ));
                }

                _ => {}
            },

            BlockType::Reduce => {}
        }
        Ok(code)
    }

    /// Generate store code for the block outputs.
    pub(crate) fn gen_store(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        if self.block_type == BlockType::Reduce {
            return Ok(code);
        }
        for edge in self.outputs.iter() {
            code += &self.emit_store(edge)?;
        }
        Ok(code)
    }

    #[allow(dead_code)]
    pub(crate) fn reduce(&self) -> Option<&Vec<Rc<AttachedEdge>>> {
        match self.block_type {
            BlockType::Reduce => Some(&self.outputs),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_mem_level(&self) -> MemoryLevel {
        self.mem_level
    }

    pub(crate) fn emit_block(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        if let Some(access_map) = &self.unified_access_map {
            let mut inner_code = String::new();

            inner_code += &self.gen_loop_load()?;
            inner_code += Sync::emit_sync().as_str();
            if self.mem_level == MemoryLevel::Shared {
                inner_code += Sync::emit_copy_async().as_str();
            }
            inner_code += self.subgraph.emit()?.as_str();
            code += access_map.gen_loop_access(inner_code)?.as_str();

            code += Sync::emit_sync().as_str();
            if let Some(reduce_outputs) = self.subgraph.reduce_block_outputs() {
                // self.outputs.extend(reduce_outputs);
                for output in reduce_outputs {
                    code += &self.emit_store(&output)?;
                }
            }

            code += &self.gen_store()?;
            Ok(code)
        } else {
            // TODO: Handle cases without an unified access map.
            if self.inputs.is_empty() && self.outputs.is_empty() {
                let code = self.subgraph.emit()?;
                Ok(code)
            } else {
                unimplemented!();
            }
        }
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
