use std::rc::Rc;

use crate::var::{IterationVar, Var};
use crate::{ThrillerError, ThrillerResult};

/// An AccessMatrix represents a multi-dimensional access pattern.
pub struct AccessMatrix(pub Vec<Vec<usize>>);

/// An AccessOffset represents a multi-dimensional access pattern.
pub struct AccessOffset(pub Vec<usize>);

/// An AccessMap represents a multi-dimensional access pattern.
#[allow(dead_code)]
pub struct AccessMap {
    loop_depth: usize,
    access_dims: Vec<usize>,
    access_matrixs: Vec<AccessMatrix>,
    offset: Vec<AccessOffset>,
    iter_vars: Vec<Rc<IterationVar>>,
}

impl AccessMap {
    /// Set the access pattern for a specific loop index.
    pub fn new(loop_depth: usize, access_dims: Vec<usize>) -> Self {
        AccessMap {
            loop_depth,
            access_dims,
            access_matrixs: vec![],
            offset: vec![],
            iter_vars: vec![],
        }
    }

    /// Add iter var to access map.
    pub fn add_iter_var(&mut self, iter_var: Rc<IterationVar>) {
        self.iter_vars.push(iter_var);
    }

    /// Get iter vars in access map.
    pub fn get_iter_vars(&self) -> &Vec<Rc<IterationVar>> {
        &self.iter_vars
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

    /// Generate loop based on `AccessMap` information.
    pub fn gen_loop_access<F>(&self, op: F) -> ThrillerResult<String>
    where
        F: Fn(&AccessMap) -> ThrillerResult<String>,
    {
        // assert_eq!(self.loop_depth, self.iter_vars.len());
        let mut code = String::new();
        let mut indent = 0;
        if self.loop_depth != self.iter_vars.len() {
            return Err(ThrillerError::LoopMisMatch);
        }
        for var in self.iter_vars.iter() {
            let (start, end) = var.get_domain();
            let name = var.get_name();

            code.push_str(&format!(
                "{indent}for(int {var} = {start}, {var} < {end}; {var}++){{\n",
                indent = " ".repeat(indent),
                var = name,
                start = start,
                end = end
            ));

            indent += 4;
        }

        let access_code = op(self)?;
        let access_lines: Vec<&str> = access_code.lines().collect();

        access_lines.iter().for_each(|line| {
            code.push_str(
                format!("{indent}{line}\n", indent = " ".repeat(indent), line = line).as_str(),
            );
        });

        for _ in 0..self.loop_depth {
            indent -= 4;
            code.push_str(format!("{indent}}}\n", indent = " ".repeat(indent)).as_str());
        }

        Ok(code)
    }
}

// impl PartialEq for AccessMap {
//     fn eq(&self, other: &Self) -> bool {
//         self.access_pattern == other.access_pattern
//             && self.offset == other.offset
//             && self.loop_depth == other.loop_depth
//             && self.access_dims == other.access_dims
//     }
// }
