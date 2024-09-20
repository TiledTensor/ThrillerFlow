use std::rc::Rc;

use thriller_core::{IterationBound, IterationVar};

use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass(unsendable, module = "var", name = "IterationVar")]
pub struct PyIterationVar(pub Rc<IterationVar>);

#[pymethods]
impl PyIterationVar {
    #[new]
    fn new(name: String, domain: &Bound<'_, PyTuple>) -> Self {
        let domain = domain.extract::<(usize, usize)>().unwrap();
        let domain_bound = (
            IterationBound::Fixed(domain.0),
            IterationBound::Fixed(domain.1),
        );
        println!("domain: {:?}", domain_bound);
        let var = IterationVar::new(&name, domain_bound);
        PyIterationVar(Rc::new(var))
    }
}
