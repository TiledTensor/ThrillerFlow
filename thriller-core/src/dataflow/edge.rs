use std::cell::RefCell;
use std::rc::Rc;

use crate::access::AccessMap;
use crate::buffer::Buffer;
use crate::dataflow::ThrillerNode;
use crate::next_id;
use crate::var::Var;

/// AttachedEdge is an edge that connects a source and destination buffer
/// with additional access pattern information `AccessMap`.
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

    pub(crate) fn check_loop_equal(&self, other: &AttachedEdge) -> bool {
        if let (Some(this), Some(other)) = (self.get_access(), other.get_access()) {
            // Check `loop_depth` is the same.
            if this.get_loop_depth() != other.get_loop_depth() {
                return false;
            }
            // Check if iter_vars are the same.
            if this.get_iter_vars().len() != other.get_iter_vars().len() {
                return false;
            }

            for (this_iter_var, other_iter_var) in this
                .get_iter_vars()
                .iter()
                .zip(other.get_iter_vars().iter())
            {
                if this_iter_var.get_id() != other_iter_var.get_id() {
                    return false;
                }
            }

            return true;
        }
        false
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

    // /// Get the source node name of the edge.
    // pub fn get_src_name(&self) -> &String {
    //     match self.get_src().borrow().get_inner() {
    //         ThrillerNodeInner::Buffer(buffer) => buffer.get_name(),
    //         _ => panic!("Source is not a buffer"),
    //     }
    // }

    // /// Get the destination node name of the edge.
    // pub fn get_dst_name(&self) -> &String {
    //     match self.dst.borrow().get_inner() {
    //         ThrillerNodeInner::Buffer(buffer) => buffer.get_name(),
    //         _ => panic!("Destination is not a buffer"),
    //     }
    // }
}
