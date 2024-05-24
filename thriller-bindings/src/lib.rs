use pyo3::prelude::*;

use buffer::PyBuffer;
use thriller_core::initialize;

mod buffer;

#[pyfunction]
fn initialize_thriller_flow() -> PyResult<()> {
    initialize();
    Ok(())
}

// #[pyfunction]
// fn create_buffer(name: String) -> PyResult<PyBuffer> {
//     let buffer = PyBuffer(Buffer::new(name.as_str()));
//     Ok(buffer)
// }

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn thriller_flow(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(initialize_thriller_flow, m)?)?;
    // m.add_function(wrap_pyfunction!(create_buffer, m)?)?;
    m.add_class::<PyBuffer>()?;
    Ok(())
}
