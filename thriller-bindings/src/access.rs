use std::rc::Rc;

use thriller_core::{AccessMap, AccessMatrix, AccessOffset};

use pyo3::{prelude::*, types::PyList};

use crate::var::PyIterationVar;

#[pyclass(unsendable, module = "access", name = "AccessMap")]
pub struct PyAccessMap(pub Rc<AccessMap>);

#[pymethods]
impl PyAccessMap {
    #[new]
    fn new(
        dims: Bound<PyList>,
        access: Bound<PyList>,
        offset: Bound<PyList>,
        vars: Bound<PyList>,
    ) -> Self {
        let dims = dims
            .into_iter()
            .map(|d| d.extract::<usize>().unwrap())
            .collect::<Vec<_>>();

        let vars = vars
            .into_iter()
            .map(|v| v.extract::<PyRef<PyIterationVar>>().unwrap().0.clone())
            .collect::<Vec<_>>();

        let access = access
            .into_iter()
            .map(|a| {
                a.extract::<Bound<PyList>>()
                    .unwrap()
                    .into_iter()
                    .map(|i| i.extract::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let access = AccessMatrix(access);

        let offset = offset
            .into_iter()
            .map(|o| o.extract::<usize>().unwrap())
            .collect::<Vec<_>>();
        let offset = AccessOffset(offset);

        let mut map = AccessMap::new(dims.len(), dims);

        map.add_iter_vars(vars);
        map.add_access_matrix(access);
        map.add_access_offset(offset);

        PyAccessMap(Rc::new(map))
    }
}
