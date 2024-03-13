use config::Config;

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct EnvAwsConfig {
    pub aws_access_key_id: Option<String>,
    pub aws_access_key: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub aws_secret_key: Option<String>,
    pub aws_session_token: Option<String>,
    pub aws_container_credentials_full_uri: Option<String>,
    pub aws_container_credentials_relative_uri: Option<String>,
    pub aws_container_authorization_token: Option<String>,
    pub aws_region: Option<String>,
    pub aws_default_region: Option<String>,
    pub aws_profile: String,
    pub aws_default_profile: Option<String>,
    pub aws_shared_credentials_file: Option<String>,
    pub aws_config_file: Option<String>,
    pub aws_ca_bundle: Option<String>,
    pub aws_web_identity_token_file: Option<String>,
    pub aws_role_arn: Option<String>,
    pub aws_role_session_name: Option<String>,
    pub aws_enable_endpoint_discovery: Option<String>,
    pub aws_ec2_metadata_service_endpoint_mode: String,
    pub aws_ec2_metadata_service_endpoint: String,
    pub aws_ec2_metadata_disabled: bool,
    pub aws_ec2_metadata_v1_disabled: bool,
    pub aws_use_dualstack_endpoint: bool,
    pub aws_use_fips_endpoint: bool,
    pub aws_defaults_mode: String,
    pub aws_max_attempts: i64,
    pub aws_retry_mode: String,
    pub aws_sdk_ua_app_id: Option<String>,
    pub aws_ignore_configured_endpoint_urls: bool,
    pub aws_endpoint_url: Option<String>,
    pub aws_disable_request_compression: bool,
    pub aws_request_min_compression_size_bytes: i64,
}

impl Default for EnvAwsConfig {
    fn default() -> Self {
        Self {
            aws_access_key_id: None,
            aws_access_key: None,
            aws_secret_access_key: None,
            aws_secret_key: None,
            aws_session_token: None,
            aws_container_credentials_full_uri: None,
            aws_container_credentials_relative_uri: None,
            aws_container_authorization_token: None,
            aws_region: None,
            aws_default_region: None,
            aws_profile: "default".to_owned(),
            aws_default_profile: None,
            aws_shared_credentials_file: None,
            aws_config_file: None,
            aws_ca_bundle: None,
            aws_web_identity_token_file: None,
            aws_role_arn: None,
            aws_role_session_name: None,
            aws_enable_endpoint_discovery: None,
            aws_ec2_metadata_service_endpoint_mode: "IPv4".to_owned(),
            aws_ec2_metadata_service_endpoint: "".to_owned(),
            aws_ec2_metadata_disabled: false,
            aws_ec2_metadata_v1_disabled: false,
            aws_use_dualstack_endpoint: false,
            aws_use_fips_endpoint: false,
            aws_defaults_mode: "standard".to_owned(),
            aws_max_attempts: 5,
            aws_retry_mode: "standard".to_owned(),
            aws_sdk_ua_app_id: None,
            aws_ignore_configured_endpoint_urls: false,
            aws_endpoint_url: None,
            aws_disable_request_compression: false,
            aws_request_min_compression_size_bytes: 10240,
        }
    }
}

impl EnvAwsConfig {
    pub fn resolve() -> Self {
        let config = Config::builder()
            .add_source(
                config::Environment::default()
                    .try_parsing(true)
                    .convert_case(config::Case::ScreamingSnake)
                    .list_separator(" "),
            )
            .build()
            .unwrap();

        let mut result: Self = config.try_deserialize().unwrap();
        match result.aws_ec2_metadata_service_endpoint_mode.as_str() {
            "IPv4" => {
                result.aws_ec2_metadata_service_endpoint = "http://169.254.169.254".to_owned();
            }
            "IPv6" => {
                result.aws_ec2_metadata_service_endpoint = "http://[fd00:ec2::254]".to_owned();
            }
            _ => {
                result.aws_ec2_metadata_service_endpoint_mode = "IPv4".to_owned();
                result.aws_ec2_metadata_service_endpoint = "http://169.254.169.254".to_owned();
            }
        }

        result
    }
}
