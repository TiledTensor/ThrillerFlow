use super::ThrillerGraph;

mod allocate_edge;
mod allocate_var;
mod gen_iterator;

pub use allocate_var::AllocateVar;

/// A trait for graph passes.
pub trait GraphPass {
    /// Run the pass on the graph.
    fn run(&mut self, graph: &mut ThrillerGraph);
}
