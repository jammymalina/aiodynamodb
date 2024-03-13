pub mod env_config;

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
        let env_config = env_config::EnvAwsConfig::resolve();

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
