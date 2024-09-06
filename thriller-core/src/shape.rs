/// Array index type.
pub type Ix = usize;

pub trait Dimension {
    /// Returns the number of dimensions (number of axes).
    fn ndim(&self) -> usize;

    /// Creates a dimension of all zeros with the specified ndim.
    fn zeros(ndim: usize) -> Self;

    fn slice(&self) -> &[Ix];

    fn slice_mut(&mut self) -> &mut [Ix];

    fn default_strides(&self) -> &[Ix];

    fn fortran_strides(&self) -> &[Ix];
}

/// Stride description.
#[derive(Clone, Copy)]
pub(crate) enum Strides<D> {
    /// Row-major
    RowMajor,
    /// Column-major
    ColumnMajor,
    // /// Custom strides
    Custom(D),
}

impl<D> Strides<D>
where
    D: Dimension,
{
    pub(crate) fn strides_for_dim(self, dim: &D) -> &[Ix] {
        match self {
            Strides::RowMajor => dim.default_strides(),
            Strides::ColumnMajor => dim.fortran_strides(),
            Strides::Custom(_) => {
                todo!()
            }
        }
    }
}

/// Dimension description.
///
/// `Dim` describes the number of axes and the length of each axis
/// in an array. It is also used as an index type.
#[derive(Clone, Copy)]
pub struct Dim {
    dims: [Ix; 4],
    ndim: usize,
}

impl Dim {
    /// Create a new Dim.
    pub fn new(dims: &[Ix]) -> Self {
        let ndim = dims.len();
        let mut new_dims = [0; 4];
        let mut index = 3;
        for dim in dims.iter().rev() {
            new_dims[index] = *dim;
            index -= 1;
        }

        Self {
            dims: new_dims,
            ndim,
        }
    }
}

impl Dimension for Dim {
    fn ndim(&self) -> usize {
        self.ndim
    }

    fn zeros(ndim: usize) -> Self {
        Self { dims: [0; 4], ndim }
    }

    fn slice(&self) -> &[Ix] {
        &self.dims[..self.ndim]
    }

    fn slice_mut(&mut self) -> &mut [Ix] {
        &mut self.dims[..self.ndim]
    }

    fn default_strides(&self) -> &[Ix] {
        todo!()
    }

    fn fortran_strides(&self) -> &[Ix] {
        todo!()
    }
}

/// Shape description.
pub struct Shape {
    dims: Dim,
    strides: Strides<Dim>,
}

impl Shape {
    pub fn new(dims: &[Ix], strides: Strides<Dim>) -> Self {
        Self {
            dims: Dim::new(dims),
            strides,
        }
    }

    /// Compute the stride for the given dimension.
    pub fn get_strides(&self) -> &[Ix] {
        self.strides.strides_for_dim(&self.dims)
    }
}
