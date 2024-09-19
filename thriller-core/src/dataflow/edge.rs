use std::cell::RefCell;
use std::rc::Rc;

use crate::access::AccessMap;
use crate::buffer::Buffer;
use crate::dataflow::ThrillerNode;
use crate::{next_id, ThrillerResult, Var};

/// [`AttachedEdge`] is an edge that connects a source and destination buffer
/// with additional access pattern information [`AccessMap`].
///
/// [`AttachedEdge`] allows connections across different nested loops
/// ([`crate::ThrillerBlock`]), fixing the relevant ivars for use
/// in the generated memory access code.
///
/// Examples:
/// In FlashAttention-v2:
/// ```cpp
/// for (int n = 0; n < GIteratorV::sc0; ++n) {
///     load_sv(gVs(n), sV);
///     for (int k = 0; k < GIteratorQ::sc1; ++k) {
///          load_sq(gQs(k), sQ);
///          load_sk(gKs(k, n), sK);
///          ...
///     }
/// }    
/// ```
/// In the above example, the `AttachedEdge` between `gKs` and `sK` will have
/// the following ivars: `n` and `k`. This is because the `gKs` buffer
/// is accessed by the outer loop `n` and the inner loop `k`.
pub struct AttachedEdge {
    #[allow(dead_code)]
    pub(crate) id: usize,
    pub(crate) src: Rc<Buffer>,
    pub(crate) dst: Rc<Buffer>,
    pub(crate) access: Rc<AccessMap>,
}

impl AttachedEdge {
    /// Create a new `AttachedEdge` with the given source and destination buffers.
    pub fn new(src: Rc<Buffer>, dst: Rc<Buffer>, access: Rc<AccessMap>) -> Self {
        AttachedEdge {
            id: next_id(),
            src,
            dst,
            access,
        }
    }

    /// Get the source buffer of the edge.
    pub fn get_src_name(&self) -> &String {
        self.src.get_name()
    }

    /// Get the destination buffer of the edge.
    pub fn get_dst_name(&self) -> &String {
        self.dst.get_name()
    }

    /// Get the access pattern of the edge.
    pub fn get_access(&self) -> &Rc<AccessMap> {
        &self.access
    }

    /// Emit Memory Access code based on index.
    pub fn emit_access(&self, index: usize) -> ThrillerResult<Vec<String>> {
        let mut access = vec![];

        let access_map = &self.access;
        let access_matrix = &access_map.access_matrixs[index];
        let access_offset = &access_map.offset[index];
        let ivars = &access_map.ivars;

        for (rindex, access_row) in access_matrix.0.iter().enumerate() {
            let offset = &access_offset.0[rindex];

            let mut code = String::new();
            // Emit the access row mulipled ivar.
            for (cindex, access_col) in access_row.iter().enumerate() {
                let ivar = &ivars[cindex];
                if *access_col != 0 {
                    code.push_str(
                        format!(
                            "{access} * {ivar}",
                            access = *access_col,
                            ivar = ivar.get_name()
                        )
                        .as_str(),
                    );
                }
            }

            if *offset != 0 {
                code.push_str(format!(" + {}", offset).as_str());
            }

            access.push(code);
        }

        Ok(access)
    }
}

/// `ThrillerEdge` repersent load/store in dataflow graph.
#[allow(dead_code)]
pub struct ThrillerEdge {
    src: Rc<RefCell<ThrillerNode>>,
    dst: Rc<RefCell<ThrillerNode>>,
}

impl ThrillerEdge {
    /// Create a new empty `ThrillerEdge`.
    pub fn new(src: Rc<RefCell<ThrillerNode>>, dst: Rc<RefCell<ThrillerNode>>) -> Self {
        ThrillerEdge { src, dst }
    }

    pub(crate) fn get_src(&self) -> Rc<RefCell<ThrillerNode>> {
        self.src.clone()
    }

    pub(crate) fn get_dst(&self) -> Rc<RefCell<ThrillerNode>> {
        self.dst.clone()
    }
}
