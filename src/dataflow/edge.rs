use std::rc::Rc;
use std::vec::Vec;

use crate::access::AccessMap;
use crate::buffer::Buffer;
use crate::dataflow::ThrillerNode;
use crate::task::Task;

/// AttachedEdge is an edge that connects a source and destination buffer
/// with additional access pattern information `AccessMap`.
pub struct AttachedEdge {
    src: Rc<Buffer>,
    dst: Rc<Buffer>,
    #[allow(dead_code)]
    access: Option<AccessMap>,
}

impl AttachedEdge {
    /// Create a new `AttachedEdge` with the given source and destination buffers.
    pub fn new(src: Rc<Buffer>, dst: Rc<Buffer>, access: Option<AccessMap>) -> Self {
        AttachedEdge { src, dst, access }
    }

    /// Get the source buffer of the edge.
    pub fn get_src_name(&self) -> &String {
        self.src.get_name()
    }

    /// Get the destination buffer of the edge.
    pub fn get_dst_name(&self) -> &String {
        self.dst.get_name()
    }
}

/// `ThrillerEdge` repersent load/store in dataflow graph.
#[allow(dead_code)]
pub struct ThrillerEdge {
    in_nodes: Vec<Rc<ThrillerNode>>,
    out_nodes: Vec<Rc<ThrillerNode>>,
    out_block: Option<Rc<ThrillerNode>>,
    access: AccessMap,
    task: Box<dyn Task>,
}

impl ThrillerEdge {
    /// Create a new empty `ThrillerEdge`.
    pub fn new(access: AccessMap, task: Box<dyn Task>) -> Self {
        ThrillerEdge {
            in_nodes: Vec::new(),
            out_nodes: Vec::new(),
            out_block: None,
            access,
            task,
        }
    }
}
