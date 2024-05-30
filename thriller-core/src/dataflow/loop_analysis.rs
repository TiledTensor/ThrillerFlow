use std::rc::Rc;

use log::info;

use crate::{next_id, AttachedEdge};

use super::block::ThrillerBlock;

pub struct LoopGroup {
    #[allow(dead_code)]
    index: usize,
    pub(crate) edges: Vec<Rc<AttachedEdge>>,
}

impl ThrillerBlock {
    /// Merge the same nest loop pattern into a loop group.
    pub fn merge_loops(&mut self) {
        let mut loop_groups = vec![];
        for input in self.get_inputs().iter() {
            if let Some(group_index) = self.check_loop_edge_equal(input, &loop_groups) {
                loop_groups[group_index].edges.push(input.clone());
            } else {
                let new_group = LoopGroup {
                    index: next_id(),
                    edges: vec![input.clone()],
                };
                loop_groups.push(new_group);
            }
        }

        self.loop_groups = loop_groups;

        info!("loop groups size: {}", self.loop_groups.len());
    }

    pub(crate) fn check_loop_edge_equal(
        &self,
        ref_edge: &Rc<AttachedEdge>,
        loop_groups: &[LoopGroup],
    ) -> Option<usize> {
        for (index, group) in loop_groups.iter().enumerate() {
            for edge in group.edges.iter() {
                if ref_edge.check_loop_equal(edge) {
                    return Some(index);
                }
            }
        }
        None
    }
}
