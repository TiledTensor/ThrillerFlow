use thriller_core::{BufType, Buffer, Layout};

/// Buffer builder.
pub struct BufBuilder();

impl BufBuilder {
    /// Create a new Row Major Global Tile buffer with the given name and dimension.
    pub fn row_major_global_tile(name: &str, dim: &[usize]) -> Buffer {
        Buffer::new(name, BufType::GlobalTile, dim, Layout::RowMajor)
    }

    /// Create a new Column Major Global Tile buffer with the given name and dimension.
    pub fn col_major_global_tile(name: &str, dim: &[usize]) -> Buffer {
        Buffer::new(name, BufType::GlobalTile, dim, Layout::ColumnMajor)
    }

    /// Create a new Row Major Shared Tile buffer with the given name and dimension.
    pub fn row_major_shared_tile(name: &str, dim: &[usize]) -> Buffer {
        Buffer::new(name, BufType::SharedTile, dim, Layout::RowMajor)
    }

    /// Create a new Column Major Shared Tile buffer with the given name and dimension.
    pub fn col_major_shared_tile(name: &str, dim: &[usize]) -> Buffer {
        Buffer::new(name, BufType::SharedTile, dim, Layout::ColumnMajor)
    }

    /// Create a new Row Major Register Tile buffer with the given name and dimension.
    pub fn row_major_reg_tile(name: &str, dim: &[usize]) -> Buffer {
        Buffer::new(name, BufType::RegTile, dim, Layout::RowMajor)
    }

    /// Create a new Column Major Register Tile buffer with the given name and dimension.
    pub fn col_major_reg_tile(name: &str, dim: &[usize]) -> Buffer {
        Buffer::new(name, BufType::RegTile, dim, Layout::ColumnMajor)
    }
}
