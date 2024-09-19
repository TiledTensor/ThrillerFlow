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

        let accesses = access
            .into_iter()
            .map(|a| {
                a.extract::<Bound<PyList>>()
                    .unwrap()
                    .into_iter()
                    .map(|i| {
                        i.extract::<Bound<PyList>>()
                            .unwrap()
                            .into_iter()
                            .map(|j| j.extract::<usize>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let access_matrixs = accesses.into_iter().map(AccessMatrix).collect::<Vec<_>>();

        let offsetes = offset
            .into_iter()
            .map(|o| {
                o.extract::<Bound<PyList>>()
                    .unwrap()
                    .into_iter()
                    .map(|i| i.extract::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let access_offsets = offsetes.into_iter().map(AccessOffset).collect::<Vec<_>>();

        let mut map = AccessMap::new(dims.len(), dims);

        map.add_iter_vars(vars);
        map.add_access_matrixs(access_matrixs);
        map.add_access_offsets(access_offsets);

        PyAccessMap(Rc::new(map))
    }
}
