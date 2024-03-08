use pyo3::prelude::*;

#[pyclass]
struct Table {
    name: String,
    region: String,
}

#[pymethods]
impl Table {
    #[new]
    fn new(name: String, region: String) -> Self {
        Table {
            name: name,
            region: region,
        }
    }
}

#[pymodule]
fn aiodynamodb(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Table>()?;
    Ok(())
}
