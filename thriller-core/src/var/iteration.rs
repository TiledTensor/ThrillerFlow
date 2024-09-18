use std::fmt::{Debug, Display};

use super::{regular::RegularVar, Var};
use crate::next_id;

/// A bound of the iteration variable.
#[derive(Clone)]
pub enum IterationBound {
    /// A fixed bound.
    Fixed(usize),
    /// A variable bound.
    Var(RegularVar),
}

impl Debug for IterationBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IterationBound::Fixed(val) => write!(f, "{}", val),
            IterationBound::Var(var) => write!(f, "{:?}", var.get_name()),
        }
    }
}

impl Display for IterationBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IterationBound::Fixed(val) => write!(f, "{}", val),
            IterationBound::Var(var) => write!(f, "{}", var.get_name()),
        }
    }
}

/// A Variable that represents a loop index.
#[derive(Clone)]
pub struct IterationVar {
    name: String,
    id: usize,
    domain: (IterationBound, IterationBound),
}

impl IterationVar {
    /// Create a new `IterationVar` with the given name.
    pub fn new(name: &str, domain: (IterationBound, IterationBound)) -> Self {
        let id = next_id();
        IterationVar {
            name: name.to_string(),
            id,
            domain,
        }
    }

    /// Create a new `IterationVar` with the random name.
    pub fn random() -> Self {
        todo!()
    }

    /// Get the domain of the iteration variable.
    pub fn get_domain(&self) -> &(IterationBound, IterationBound) {
        &self.domain
    }
}

impl Var for IterationVar {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
