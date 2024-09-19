use std::cell::RefCell;
use std::rc::Rc;

use crate::access::AccessMap;
use crate::buffer::Buffer;
use crate::dataflow::ThrillerNode;
use crate::next_id;

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
    pub(crate) access: Option<Rc<AccessMap>>,
}

impl AttachedEdge {
    /// Create a new `AttachedEdge` with the given source and destination buffers.
    pub fn new(src: Rc<Buffer>, dst: Rc<Buffer>, access: Option<Rc<AccessMap>>) -> Self {
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
    pub fn get_access(&self) -> &Option<Rc<AccessMap>> {
        &self.access
    }

    /// Replace the access pattern of the edge.
    pub fn replace_access_map(&mut self, access: Rc<AccessMap>) {
        self.access = Some(access);
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
