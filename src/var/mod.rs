mod iteration;

pub use iteration::IterationVar;
/// A trait for variable.
pub trait Var {
    /// Get the name of the variable.
    fn get_name(&self) -> &String;

    /// Get the id of the variable.
    fn get_id(&self) -> usize;
}
