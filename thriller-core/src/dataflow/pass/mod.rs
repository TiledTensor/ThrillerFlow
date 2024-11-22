use super::ThrillerGraph;

mod allocate_edge;
mod allocate_var;
mod gen_iterator;

/// A trait for graph passes.
pub trait GraphPass {
    fn run(&mut self, graph: &mut ThrillerGraph);
}
