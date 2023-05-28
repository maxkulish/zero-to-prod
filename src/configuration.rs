use config::{ConfigError, Config};


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

// The above code is a Rust function that reads a YAML configuration file named
// "configuration.yaml" and returns a  `Result`  containing a  `Settings`
// struct or a  `ConfigError`  if there was an issue with the configuration.
// It uses the  `config`  crate to initialize and read the configuration values,
// and attempts to deserialize them into the  `Settings`  struct.
pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Initialize our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}