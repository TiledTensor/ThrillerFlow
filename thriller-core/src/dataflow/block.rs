use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::dataflow::{AttachedEdge, ThrillerGraph};
use crate::error::{ThrillerError, ThrillerResult};
use crate::kernels::sync::Sync;
use crate::task::Task;
use crate::var::Var;
use crate::{next_id, BufType, IterationBound, IterationVar};

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
    pub(crate) subgraph: Rc<RefCell<ThrillerGraph>>,
    pub(crate) block_type: BlockType,
    pub(crate) ivars: Vec<Rc<IterationVar>>,
}

impl ThrillerBlock {
    /// Create a new [`ThrillerBlock`] with the given inputs, outputs, memory level, subgraph, and block type.
    pub fn new(
        inputs: Vec<Rc<AttachedEdge>>,
        outputs: Vec<Rc<AttachedEdge>>,
        subgraph: Rc<RefCell<ThrillerGraph>>,
        block_type: BlockType,
        ivars: Vec<Rc<IterationVar>>,
    ) -> Self {
        ThrillerBlock {
            inputs,
            outputs,
            subgraph,
            block_type,
            ivars,
            id: next_id(),
        }
    }

    /// Get the block type.
    pub fn get_block_type(&self) -> BlockType {
        self.block_type
    }

    fn emit_loop(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        let mut indent = 0;

        // Generate loop.
        for ivar in self.ivars.iter() {
            let (upper, lower) = ivar.get_domain();

            code += match (upper, lower) {
                (IterationBound::Fixed(upper), IterationBound::Fixed(lower)) => {
                    format!(
                        "{indent}for(int {ivar} = {lower}; {ivar} < {upper}; {ivar}++){{\n",
                        indent = " ".repeat(indent),
                        ivar = ivar.get_name(),
                        lower = lower,
                        upper = upper
                    )
                }

                _ => todo!(),
            }
            .as_str();

            indent += 4;
        }

        Ok(code)
    }

    fn emit_loop_closure(&self) -> ThrillerResult<String> {
        let mut indent = ((self.ivars.len() - 1) * 4) as isize;
        let mut code = String::new();

        while indent >= 0 {
            code += format!("{indent}}}\n", indent = " ".repeat(indent as usize)).as_str();
            indent -= 4;
        }

        Ok(code)
    }

    fn emit_load(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        let indent = " ".repeat(self.ivars.len() * 4);

        let mut insert_copy_async = false;
        let mut insert_syncthreads = false;

        for edge in self.inputs.iter() {
            // Insert `syncthreads()` when loading tiles.
            insert_syncthreads = true;

            let sbuf = &edge.src;
            let dbuf = &edge.dst;

            // TODO(KuangjuX): Support Access Memory code generation.
            #[allow(unused_variables)]
            let access_map = edge
                .get_access()
                .as_ref()
                .ok_or(ThrillerError::MissingAccessMap)?;

            let sbuf_var = sbuf.get_name();
            let dbuf_var = dbuf.get_name();

            let sbuf_id = sbuf.get_id();
            let dbuf_id = dbuf.get_id();

            match (sbuf.get_typing(), dbuf.get_typing()) {
                (BufType::GlobalTile, BufType::RegTile) => {
                    code += format!(
                        "{indent}loader_tile_g2r_{sid}_to_{did}({sbuf_var}, {dbuf_var});\n",
                        indent = indent,
                        sid = sbuf_id,
                        did = dbuf_id,
                        sbuf_var = sbuf_var,
                        dbuf_var = dbuf_var
                    )
                    .as_str();
                }

                (BufType::SharedTile, BufType::RegTile) => {
                    insert_copy_async = true;
                    code += format!(
                        "{indent}loader_tile_s2r_{sid}_to_{did}({sbuf_var}, {dbuf_var});\n",
                        indent = indent,
                        sid = sbuf_id,
                        did = dbuf_id,
                        sbuf_var = sbuf_var,
                        dbuf_var = dbuf_var
                    )
                    .as_str();
                }

                _ => todo!(),
            }
        }

        if insert_copy_async {
            code += format!(
                "{indent}{copy_async}",
                indent = indent,
                copy_async = Sync::emit_copy_async()
            )
            .as_str();
        }

        if insert_syncthreads {
            code += format!(
                "{indent}{syncthreads}",
                indent = indent,
                syncthreads = Sync::emit_sync()
            )
            .as_str();
        }

        Ok(code)
    }

    fn emit_store(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        for edge in self.outputs.iter() {
            let sbuf = &edge.src;
            let dbuf = &edge.dst;

            // TODO(KuangjuX): Support Access Memory code generation.
            #[allow(unused_variables)]
            let access_map = edge
                .get_access()
                .as_ref()
                .ok_or(ThrillerError::MissingAccessMap)?;

            let sbuf_var = sbuf.get_name();
            let dbuf_var = dbuf.get_name();

            let sbuf_id = sbuf.get_id();
            let dbuf_id = dbuf.get_id();

            match (sbuf.get_typing(), dbuf.get_typing()) {
                (BufType::RegTile, BufType::GlobalTile) => {
                    code += format!(
                        "storer_tile_r2g_{sid}_to_{did}({sbuf_var}, {dbuf_var});\n",
                        sid = sbuf_id,
                        did = dbuf_id,
                        sbuf_var = sbuf_var,
                        dbuf_var = dbuf_var
                    )
                    .as_str();
                }

                _ => todo!(),
            }
        }

        Ok(code)
    }

    fn emit_sync(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        // TODO(KuangjuX): Check Memory Hiercary and insert sync primitive.
        code += Sync::emit_sync().as_str();

        Ok(code)
    }

    pub(crate) fn emit_block(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        code += self.emit_loop()?.as_str();
        let indent = " ".repeat(self.ivars.len() * 4);

        code += self.emit_load()?.as_str();

        let subgraph_code = self.subgraph.borrow().emit()?;

        for line in subgraph_code.lines() {
            code += format!("{indent}{line}\n", indent = indent, line = line).as_str()
        }

        code += self.emit_loop_closure()?.as_str();

        code += self.emit_sync()?.as_str();

        code += self.emit_store()?.as_str();

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
