use hot_reload::HotReloadConfig;
use serde::de::DeserializeOwned;
use crate::{error::{ConfigError, ErrorCause}, types::ConfigResult};

mod hot_reload;

pub struct ConfigManager {
    config_type: ConfigType,
    hot_reload: Option<HotReloadConfig>
}

pub enum ConfigType {
    Json,
    Yaml,
    Toml
}


impl ConfigManager {
    pub fn new_from_path<T: DeserializeOwned>(config_type: ConfigType, path: &str) -> ConfigResult<T> {
        Self::init_by_type_from_path::<T>(config_type, path)
    }

    pub fn new_from_str<T: DeserializeOwned>(config_type: ConfigType, input: &str) -> ConfigResult<T> {
        Self::init_by_type_from_str(config_type, input)
    }

    fn init_by_type_from_path<T: DeserializeOwned>(config_type: ConfigType, path: &str) -> ConfigResult<T> {
        match config_type {
            ConfigType::Json => {
                #[cfg(not(feature = "json"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Json feature is not enabled".to_string())));
                #[cfg(feature = "json")]
                {
                    let config = std::fs::read_to_string(path)
                    .map_err(| error| ConfigError::new(ErrorCause::FileOpen, Some(error.to_string())))?;
                    serde_json::from_str::<T>(&config)
                        .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
                }
            },
            ConfigType::Yaml => {
                #[cfg(not(feature = "yaml"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Yaml feature is not enabled".to_string())));
                #[cfg(feature = "yaml")]
                {
                    let config = std::fs::read_to_string(path)
                        .map_err(| error| ConfigError::new(ErrorCause::FileOpen, Some(error.to_string())))?;
                    serde_yaml::from_str::<T>(&config)
                        .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
                }
            },
            ConfigType::Toml => {
                #[cfg(not(feature = "toml"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Toml feature is not enabled".to_string())));
                #[cfg(feature = "toml")]
                {
                    let config = std::fs::read_to_string(path)
                        .map_err(| error| ConfigError::new(ErrorCause::FileOpen, Some(error.to_string())))?;
                    toml::from_str::<T>(&config)
                        .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
                }
            },
        }
    }

    fn init_by_type_from_str<T: DeserializeOwned>(config_type: ConfigType, input: &str) -> ConfigResult<T> {
        match config_type {
            ConfigType::Json => {
                #[cfg(not(feature = "json"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Json feature is not enabled".to_string())));
                #[cfg(feature = "json")]
                serde_json::from_str::<T>(input)
                    .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
            },
            ConfigType::Yaml => {
                #[cfg(not(feature = "yaml"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Yaml feature is not enabled".to_string())));
                #[cfg(feature = "yaml")]
                serde_yaml::from_str(input)
                    .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
            },
            ConfigType::Toml => {
                #[cfg(not(feature = "toml"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Toml feature is not enabled".to_string())));
                #[cfg(feature = "toml")]
                toml::from_str(input)
                    .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
            },
        }
    }

    pub fn with_hot_reload(self) -> ConfigResult<Self> {
        unimplemented!();

        // #[cfg(not(feature = "hot_reload"))]
        // return Err(ConfigError(String::from("Hot reload is disabled by feature flag")));

        //  TODO complete configuration and implementation

        // Ok(self)
    }

    pub fn with_cron(self) -> ConfigResult<Self> {
        unimplemented!()

        //  TODO
    }
}
