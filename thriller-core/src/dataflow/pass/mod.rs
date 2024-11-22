use super::ThrillerGraph;

mod allocate_var;

/// A trait for graph passes.
pub trait GraphPass {
    fn run(&mut self, graph: &mut ThrillerGraph);
}
