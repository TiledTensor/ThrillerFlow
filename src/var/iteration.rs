use super::Var;
use crate::next_id;

/// A Variable that represents a loop index.
pub struct IterationVar {
    name: String,
    id: usize,
    domain: (usize, usize),
}

impl IterationVar {
    /// Create a new `IterationVar` with the given name.
    pub fn new(name: &str, domain: (usize, usize)) -> Self {
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
    pub fn get_domain(&self) -> (usize, usize) {
        self.domain
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
