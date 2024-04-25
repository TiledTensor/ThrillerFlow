use std::rc::Rc;

use crate::dataflow::graph::ThrillerGraph;
use crate::task::Task;

/// A Thriller Dataflow Node that represents either a block of subgraph or an operation.
pub enum ThrillerNode {
    /// Represents a block of subgraph.
    Block(Rc<ThrillerGraph>),
    /// Represents an operation.
    Op(Rc<Box<dyn Task>>),
}
