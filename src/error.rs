/// Errors that can occur in the thriller crate.
#[derive(Debug)]
pub enum ThrillerError {
    /// The given access pattern is invalid.
    InvalidAccessPattern,
    /// The given loop mismatch.
    LoopMisMatch,
}

/// Result type for thriller crate functions.
pub type ThrillerResult<T> = Result<T, ThrillerError>;
