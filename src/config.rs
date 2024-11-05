use serde_aux::field_attributes::deserialize_number_from_string;
use std::convert::{TryFrom, TryInto};

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub mem_table: MemTableImplSettings,
    pub env: Environment,
}

#[derive(serde::Deserialize, Clone)]
pub struct VectorMemTableSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub initial_vec_size: u128,
}

impl Default for VectorMemTableSettings {
    fn default() -> Self {
        Self {
            initial_vec_size: 1024,
        }
    }
}

#[derive(serde::Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MemTableImplSettings {
    VectorMemTable(VectorMemTableSettings),
}

#[derive(serde::Deserialize, Clone)]
pub struct MemTableSettings {
    #[serde(flatten)]
    pub table_impl: MemTableImplSettings,
    pub write_buffer_size_in_mb: u128,
}

impl Default for MemTableSettings {
    fn default() -> Self {
        Self {
            table_impl: MemTableImplSettings::VectorMemTable(VectorMemTableSettings::default()),
            write_buffer_size_in_mb: Default::default(),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("SALOON")
                .prefix_separator("_")
                .separator("_"),
        )
        .build()?;

    let mut settings_parsed = settings.try_deserialize::<Settings>()?;

    settings_parsed.env = environment;

    Ok(settings_parsed)
}

/// The possible runtime environment for our application.
#[derive(Clone, serde::Deserialize)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
