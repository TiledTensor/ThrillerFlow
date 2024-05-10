mod iteration;
mod regular;

pub use iteration::{IterationBound, IterationVar};
pub use regular::RegularVar;
/// A trait for variable.
pub trait Var {
    /// Get the name of the variable.
    fn get_name(&self) -> &String;

    /// Get the id of the variable.
    fn get_id(&self) -> usize;
}
