use std::rc::Rc;

use crate::var::IterationVar;

/// An [`AccessMatrix`] represents a multi-dimensional access pattern.
pub struct AccessMatrix(pub Vec<Vec<usize>>);

/// An [`AccessOffset`] represents a multi-dimensional access pattern.
pub struct AccessOffset(pub Vec<usize>);

/// An [`AccessMap`] represents a multi-dimensional access pattern.
///
/// An [`AccessMap`] data structure is attached to an [`crate::AttachedEdge`]
/// and represents the memoey access patterns of the source [`crate::Buffer`]
/// and the target [`crate::Buffer`].
///
/// It refers from polyhedral mathematical model for analyzing memory access patterns.
pub struct AccessMap {
    loop_depth: usize,
    #[allow(dead_code)]
    access_dims: Vec<usize>,
    access_matrixs: Vec<AccessMatrix>,
    offset: Vec<AccessOffset>,
    ivars: Vec<Rc<IterationVar>>,
}

impl AccessMap {
    /// Set the access pattern for a specific loop index.
    pub fn new(loop_depth: usize, access_dims: Vec<usize>) -> Self {
        AccessMap {
            loop_depth,
            access_dims,
            access_matrixs: vec![],
            offset: vec![],
            ivars: vec![],
        }
    }

    /// Add iter var to access map.
    pub fn add_iter_var(&mut self, iter_var: Rc<IterationVar>) {
        self.ivars.push(iter_var);
    }

    /// Add iter vars to access map.
    pub fn add_iter_vars(&mut self, iter_vars: Vec<Rc<IterationVar>>) {
        self.ivars.extend(iter_vars);
    }

    /// Get iter vars in access map.
    pub fn get_iter_vars(&self) -> &Vec<Rc<IterationVar>> {
        &self.ivars
    }

    /// Add an access matrix to the access map.
    pub fn add_access_matrix(&mut self, access_matrix: AccessMatrix) {
        self.access_matrixs.push(access_matrix);
    }

    /// Add an access offset to the access map.
    pub fn add_access_offset(&mut self, access_offset: AccessOffset) {
        self.offset.push(access_offset);
    }

    /// Get access matrixs in access map.
    pub fn get_access_matrixs(&self) -> &Vec<AccessMatrix> {
        &self.access_matrixs
    }

    /// Get access offsets in access map.
    pub fn get_access_offsets(&self) -> &Vec<AccessOffset> {
        &self.offset
    }

    /// Get Loop depth of AccessMap
    pub fn get_loop_depth(&self) -> usize {
        self.loop_depth
    }
}
