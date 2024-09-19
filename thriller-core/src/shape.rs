use smallvec::{smallvec, SmallVec};

/// Array index type.
pub type Ix = usize;

/// Dimension description.
pub trait Dimension {
    /// Returns the number of dimensions (number of axes).
    fn ndim(&self) -> usize;

    /// Creates a dimension of all zeros with the specified ndim.
    fn zeros(ndim: usize) -> Self;

    #[doc(hidden)]
    fn slice(&self) -> &[Ix];

    #[doc(hidden)]
    fn slice_mut(&mut self) -> &mut [Ix];

    ///  returns the strides for a standard layout with the given shape.
    fn default_strides(&self) -> Self;

    /// Returns the strides for a Fortran layout array with the given shape.
    fn fortran_strides(&self) -> Self;
}

/// Stride description.
#[derive(Clone, Copy, Debug)]
pub enum Layout<D> {
    /// Row-major
    RowMajor,
    /// Column-major
    ColumnMajor,
    /// Custom strides
    Custom(D),
}

impl<D> Layout<D>
where
    D: Dimension,
{
    pub(crate) fn strides_for_dim(self, dim: &D) -> D {
        match self {
            Layout::RowMajor => dim.default_strides(),
            Layout::ColumnMajor => dim.fortran_strides(),
            Layout::Custom(_) => {
                todo!()
            }
        }
    }
}

/// Dimension description.
///
/// [`Dim`] describes the number of axes and the length of each axis
/// in an array. It is also used as an index type.
#[derive(Clone, Debug)]
pub struct Dim {
    dims: SmallVec<[Ix; 4]>,
    ndim: usize,
}

impl Dim {
    /// Create a new Dim.
    pub fn new(dims: &[Ix]) -> Self {
        let ndim = dims.len();
        let mut new_dims = smallvec![];
        for dim in dims.iter() {
            new_dims.push(*dim);
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
        Self {
            dims: smallvec![0; ndim],
            ndim,
        }
    }

    fn slice(&self) -> &[Ix] {
        &self.dims[..self.ndim]
    }

    fn slice_mut(&mut self) -> &mut [Ix] {
        &mut self.dims[..self.ndim]
    }

    fn default_strides(&self) -> Self {
        // Compute default strides
        // Shape (a, b, c) => Give strides (b * c, c, 1)
        let mut strides = Self::zeros(self.ndim);
        // For empty arrays, use all zero strides
        if self.slice().iter().all(|&d| d != 0) {
            let mut it = strides.slice_mut().iter_mut().rev();

            if let Some(rs) = it.next() {
                *rs = 1;
            }

            let mut cum_prod = 1;
            for (rs, dim) in it.zip(self.slice().iter().rev()) {
                cum_prod *= dim;
                *rs = cum_prod;
            }
        }

        strides
    }

    /// Returns the strides for a Fortran layout array with the given shape.
    fn fortran_strides(&self) -> Self {
        // Compute fortan array strides
        // Shape (a, b, c) => Give strides (1, a, a * b)
        let mut strides = Self::zeros(self.ndim);
        // For empty arrays, use all zero strides
        if self.slice().iter().all(|&d| d != 0) {
            let mut it = strides.slice_mut().iter_mut();
            if let Some(rs) = it.next() {
                *rs = 1;
            }
            let mut cum_prod = 1;
            for (rs, dim) in it.zip(self.slice().iter()) {
                cum_prod *= dim;
                *rs = cum_prod;
            }
        }
        strides
    }
}

/// Shape description.
#[derive(Clone, Debug)]
pub struct Shape {
    dims: Dim,
    layout: Layout<Dim>,
}

impl Shape {
    /// Create a new Shape.
    pub fn new(dims: &[Ix], layout: Layout<Dim>) -> Self {
        Self {
            dims: Dim::new(dims),
            layout,
        }
    }

    /// Compute the stride for the given dimension.
    pub fn get_strides(&self) -> Dim {
        self.layout.clone().strides_for_dim(&self.dims)
    }
}
