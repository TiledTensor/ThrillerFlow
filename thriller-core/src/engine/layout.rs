use crate::{var::RegularVar, Var};

/// Block Shape
pub enum BlockShape {
    /// Num
    Num(usize),
    /// Var
    Var(RegularVar),
}

/// Layout configuration of a block in 3D space.
pub struct BlockLayout {
    dim3: [BlockShape; 3],
}

impl BlockLayout {
    /// Create a new block layout with the given dimensions.
    pub fn new(dim3: [BlockShape; 3]) -> Self {
        BlockLayout { dim3 }
    }

    /// Get the dimenstions.
    pub fn get_dim3(&self) -> &[BlockShape; 3] {
        &self.dim3
    }

    /// Get the x dimension
    pub fn get_dim_x(&self) -> String {
        match &self.dim3[0] {
            BlockShape::Num(num) => num.to_string(),
            BlockShape::Var(var) => var.get_name().clone(),
        }
    }

    /// Get the y dimension
    pub fn get_dim_y(&self) -> String {
        match &self.dim3[1] {
            BlockShape::Num(num) => num.to_string(),
            BlockShape::Var(var) => var.get_name().clone(),
        }
    }

    /// Get the z dimension
    pub fn get_dim_z(&self) -> String {
        match &self.dim3[2] {
            BlockShape::Num(num) => num.to_string(),
            BlockShape::Var(var) => var.get_name().clone(),
        }
    }
}
