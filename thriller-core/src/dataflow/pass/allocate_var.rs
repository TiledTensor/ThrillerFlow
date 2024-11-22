use super::GraphPass;
use crate::dataflow::ThrillerGraph;

pub struct AllocateVar;

impl GraphPass for AllocateVar {
    fn run(&mut self, graph: &mut ThrillerGraph) {}
}
