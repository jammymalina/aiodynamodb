pub mod env_config;

use pyo3::prelude::*;

#[pyclass]
struct Table {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    region: String,
    #[pyo3(get)]
    endpoint_url: String,
}

#[pymethods]
impl Table {
    #[new]
    #[pyo3(signature = (name, /, region=None, endpoint_url=None))]
    fn new(name: String, region: Option<String>, endpoint_url: Option<String>) -> Self {
        let env_config = env_config::EnvAwsConfig::resolve();

        let region_options = vec![
            region,
            env_config.aws_region,
            env_config.aws_default_region,
            Some("us-east-1".to_owned()),
        ];
        let region: String = region_options.into_iter().find_map(|x| x).unwrap();

        let default_endpoint = if env_config.aws_use_fips_endpoint {
            Some(format!("dynamodb-fips.{region}.amazonaws.com"))
        } else {
            Some(format!("dynamodb.{region}.amazonaws.com"))
        };
        let endpoint_options = vec![
            endpoint_url,
            env_config.aws_endpoint_url_dynamodb,
            env_config.aws_endpoint_url,
            default_endpoint,
        ];
        let endpoint_url: String = endpoint_options.into_iter().find_map(|x| x).unwrap();

        Table {
            name,
            region,
            endpoint_url,
        }
    }
}

#[pymodule]
fn aiodynamodb(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Table>()?;
    Ok(())
}
