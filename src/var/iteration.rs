use super::Var;
use crate::next_id;

/// A Variable that represents a loop index.
pub struct IterationVar {
    name: String,
    id: usize,
}

impl IterationVar {
    /// Create a new InterationVar with the given name.
    pub fn new(name: &str) -> Self {
        let id = next_id();
        IterationVar {
            name: name.to_string(),
            id,
        }
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
