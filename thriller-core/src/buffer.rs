use crate::shape::Ix;
use crate::{next_id, Dim, Layout, Shape};

/// Buffer type.
#[derive(Clone, Copy, Debug)]
pub enum BufType {
    /// Global Tile
    GlobalTile,
    /// Shared Tile
    SharedTile,
    /// Register Tile
    RegTile,
    /// Register Vector
    RegVec,
}

/// A Buffer data structure.
#[allow(dead_code)]
pub struct Buffer {
    name: String,
    id: usize,
    typing: BufType,
    shape: Shape,
}

impl Buffer {
    /// Create a new Buffer with the given name.
    pub fn new(name: &str, typing: BufType, dim: &[Ix], layout: Layout<Dim>) -> Self {
        let id = next_id();
        Buffer {
            name: name.to_string(),
            id,
            typing,
            shape: Shape::new(dim, layout),
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

    /// Get Buffer Layout.
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    /// Get Buffer Typing
    pub fn get_typing(&self) -> &BufType {
        &self.typing
    }
}
