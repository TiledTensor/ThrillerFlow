use crate::id::ID_COUNTER;

/// A Buffer data structure.
#[allow(dead_code)]
pub struct Buffer {
    name: String,
    id: usize,
}

impl Buffer {
    /// Create a new Buffer with the given name.
    pub fn new(name: &str) -> Self {
        let id = unsafe { ID_COUNTER.get_mut().unwrap().next() };
        Buffer {
            name: name.to_string(),
            id,
        }
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }
}
