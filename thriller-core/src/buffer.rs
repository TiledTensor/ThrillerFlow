use crate::next_id;

// /// Tile typing description.
// pub enum TileTyping {
//     GlobalTile,
//     SharedTile,
//     RegTile,
//     RegVec,
// }

/// A Buffer data structure.
#[allow(dead_code)]
pub struct Buffer {
    name: String,
    id: usize,
}

impl Buffer {
    /// Create a new Buffer with the given name.
    pub fn new(name: &str) -> Self {
        let id = next_id();
        Buffer {
            name: name.to_string(),
            id,
        }
    }

    /// Get Buffer name.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get Buffer id.
    pub fn get_id(&self) -> usize {
        self.id
    }
}
