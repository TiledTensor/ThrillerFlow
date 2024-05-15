use crate::{next_id, Var};

/// A regular variable.
pub struct RegularVar {
    name: String,
    id: usize,
}

impl RegularVar {
    /// Create a new regular variable.
    pub fn new(name: String) -> Self {
        RegularVar {
            name,
            id: next_id(),
        }
    }
}

impl Var for RegularVar {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
