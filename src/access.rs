use std::rc::Rc;

use crate::var::IterationVar;

/// An AccessMap represents a multi-dimensional access pattern.
#[allow(dead_code)]
pub struct AccessMap {
    loop_depth: usize,
    access_dims: usize,
    access_pattern: Vec<Vec<usize>>,
    offset: Vec<usize>,
    iter_var: Vec<Rc<IterationVar>>,
}

impl AccessMap {
    /// Set the access pattern for a specific loop index.
    pub fn new(loop_depth: usize, access_dims: usize) -> Self {
        AccessMap {
            loop_depth,
            access_dims,
            access_pattern: vec![vec![0; access_dims]; loop_depth],
            offset: vec![0; loop_depth],
            iter_var: vec![],
        }
    }

    /// Add iter var to access map.
    pub fn add_iter_var(&mut self, iter_var: Rc<IterationVar>) {
        self.iter_var.push(iter_var);
    }
}

impl PartialEq for AccessMap {
    fn eq(&self, other: &Self) -> bool {
        self.access_pattern == other.access_pattern
            && self.offset == other.offset
            && self.loop_depth == other.loop_depth
            && self.access_dims == other.access_dims
    }
}
