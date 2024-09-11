use pyo3::prelude::*;

use block::{PyAttachedEdge, PyBlock, PyBlockType};
use buffer::{PyBufType, PyBuffer, PyLayout};
use graph::{PyEdge, PyGraph, PyMemoryLevel, PyNode};
use op::PyGemm;
use thriller_core::initialize;

mod block;
mod buffer;
mod graph;
mod op;

#[pyfunction]
fn initialize_thriller_flow() -> PyResult<()> {
    initialize();
    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn thriller_flow(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(initialize_thriller_flow, m)?)?;
    m.add_class::<PyBuffer>()?;
    m.add_class::<PyLayout>()?;
    m.add_class::<PyBufType>()?;

    m.add_class::<PyGraph>()?;
    m.add_class::<PyNode>()?;
    m.add_class::<PyEdge>()?;
    m.add_class::<PyMemoryLevel>()?;

    m.add_class::<PyGemm>()?;

    m.add_class::<PyBlock>()?;
    m.add_class::<PyAttachedEdge>()?;
    m.add_class::<PyBlockType>()?;

    Ok(())
}
