#[derive(Debug, serde::Deserialize, Clone)]
pub struct Db {
    pub db_uri: String,
    pub pool_size: u32,
    pub run_migrations: bool,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Server {
    #[serde(default = "Server::default_debug")]
    pub debug: bool,
    pub jwt_secret_key: String,
    pub jwt_schema_key: String,
    // expires in seconds
    pub jwt_token_exp: i64,
}

impl Server {
    fn default_debug() -> bool {
        false
    }
}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[serde(rename_all_fields = "lowercase")]
pub enum TracingLevelFilter {
    Off,
    Trace,
    Debug,
    Warn,
    Error,
    Info,
}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[serde(rename_all_fields = "lowercase")]
pub enum TracingFormat {
    Pretty,
    Json,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Tracing {
    pub format: TracingFormat,
    pub level: TracingLevelFilter,
    pub off: Option<Vec<String>>,
    pub debug: Option<Vec<String>>,
    pub error: Option<Vec<String>>,
    pub warn: Option<Vec<String>>,
    pub info: Option<Vec<String>>,
    pub trace: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Config {
    pub db: Db,
    pub server: Server,
    pub tracing: Tracing,
}

impl Config {
    /// Loads configuration from a specified file and merges it with environment variables.
    ///
    /// The method first attempts to load the configuration from the given file. Supported file
    /// formats include JSON, YAML, TOML, etc. It then overrides these settings with any specified
    /// environment variables, using a double underscore `__` as a separator.
    ///
    /// # Errors
    /// Returns `config::ConfigError` if:
    /// - The file specified by `path` cannot be found, accessed, or is not properly formatted.
    /// - There are issues deserializing the configuration into the `Config` struct, such as
    ///   missing required fields or type mismatches between the file and struct definitions.
    /// - Environment variables cannot be processed correctly, either due to parsing issues or
    ///   structural mismatches with the expected configuration format.
    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

/// Loads configuration from a specified file and merges it with environment variables.
///
/// # Errors
/// Returns `config::ConfigError` if:
/// - The file specified by `path` cannot be found, accessed, or is not properly formatted.
/// - There are issues deserializing the configuration into the `Config` struct, such as
///   missing required fields or type mismatches between the file and struct definitions.
/// - Environment variables cannot be processed correctly, either due to parsing issues or
///   structural mismatches with the expected configuration format.
pub fn from_file(config_file: &str) -> Result<Config, config::ConfigError> {
    Config::from_file(config_file)
}
