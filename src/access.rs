use std::vec::Vec;

/// An AccessMap represents a multi-dimensional access pattern.
#[allow(dead_code)]
pub struct AccessMap {
    loop_depth: usize,
    access_dims: usize,
    access_pattern: Vec<Vec<usize>>,
    offset: Vec<usize>,
}

impl AccessMap {
    /// Set the access pattern for a specific loop index.
    pub fn new(loop_depth: usize, access_dims: usize) -> Self {
        AccessMap {
            loop_depth,
            access_dims,
            access_pattern: vec![vec![0; access_dims]; loop_depth],
            offset: vec![0; loop_depth],
        }
    }
}
