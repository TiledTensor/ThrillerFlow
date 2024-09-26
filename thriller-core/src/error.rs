/// Errors that can occur in the thriller crate.
#[derive(Debug)]
pub enum ThrillerError {
    /// The given access pattern is invalid.
    InvalidAccessPattern,
    /// The given loop mismatch.
    LoopMisMatch,
    /// The access map is missing.
    MissingAccessMap,
    /// Invalid Load Access Pattern.
    InvalidLoadAccess,
    /// Wrong Inputs Number.
    WrongInputsNum,
    /// Failed to perform file operation.
    FailedFileOp,
    /// Failed to parse the gieven string.
    ParseError,
}

/// Result type for thriller crate functions.
pub type ThrillerResult<T> = Result<T, ThrillerError>;
