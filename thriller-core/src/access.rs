use std::rc::Rc;

use crate::{var::IterationVar, ThrillerResult, Var};

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
    pub(crate) loop_depth: usize,
    #[allow(dead_code)]
    pub(crate) access_dims: Vec<usize>,
    pub(crate) access_matrixs: Vec<AccessMatrix>,
    pub(crate) offset: Vec<AccessOffset>,
    pub(crate) ivars: Vec<Rc<IterationVar>>,
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

    /// Add access matrixs to the access map.
    pub fn add_access_matrixs(&mut self, access_matrixs: Vec<AccessMatrix>) {
        self.access_matrixs.extend(access_matrixs);
    }

    /// Add an access offset to the access map.
    pub fn add_access_offset(&mut self, access_offset: AccessOffset) {
        self.offset.push(access_offset);
    }

    /// Add access offsets to the access map.
    pub fn add_access_offsets(&mut self, access_offsets: Vec<AccessOffset>) {
        self.offset.extend(access_offsets);
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

    /// Emit Memory Access code based on index.
    pub fn emit_access(&self, index: usize) -> ThrillerResult<Vec<String>> {
        let mut access = vec![];

        let access_matrix = &self.access_matrixs[index];
        let access_offset = &self.offset[index];
        let ivars = &self.ivars;

        for (rindex, access_row) in access_matrix.0.iter().enumerate() {
            let offset = &access_offset.0[rindex];

            let mut code = String::new();
            // Emit the access row mulipled ivar.
            for (cindex, access_col) in access_row.iter().enumerate() {
                let ivar = &ivars[cindex];
                code.push_str(
                    format!(
                        "{access}*{ivar}",
                        access = *access_col,
                        ivar = ivar.get_name()
                    )
                    .as_str(),
                );
            }

            if *offset != 0 {
                code.push_str(format!("+{}", offset).as_str());
            }

            access.push(code);
        }

        Ok(access)
    }
}
