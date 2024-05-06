/// Errors that can occur in the thriller crate.
pub enum ThrillerError {
    /// The given access pattern is invalid.
    InvalidAccessPattern,
}

/// Result type for thriller crate functions.
pub type ThrillerResult<T> = Result<T, ThrillerError>;
