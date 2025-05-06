use serde::de::DeserializeOwned;

use crate::{error::{ConfigError, ErrorCause}, types::ConfigResult};

pub struct ConfigManager {
    config_type: ConfigType,
    hot_reload: Option<HotReloadConfig>
}

// enum ConfigContent {
//     Json(),
//     Yaml(),
//     Toml(),
// }

pub enum ConfigType {
    Json,
    Yaml,
    Toml
}

struct HotReloadConfig {

}


impl ConfigManager {
    pub fn init_from_path<T: DeserializeOwned>(config_type: ConfigType, path: &str) -> ConfigResult<T> {
        Self::validate_by_type(&config_type)?;
        Self::init_by_type_from_path::<T>(config_type, path)
    }

    pub fn new_from_str<T: DeserializeOwned>(config_type: ConfigType, input: &str) -> ConfigResult<T> {
        Self::validate_by_type(&config_type)?;
        Self::init_by_type_from_str(config_type, input)
    }

    fn validate_by_type(config_type: &ConfigType) -> ConfigResult<()> {
        match config_type {
            ConfigType::Json => {
                #[cfg(not(feature = "json"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Json feature is not enabled".to_string())))


            },
            ConfigType::Yaml => {
                #[cfg(not(feature = "yaml"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Yaml feature is not enabled".to_string())))
            },
            ConfigType::Toml => {
                #[cfg(not(feature = "toml"))]
                return Err(ConfigError::new(ErrorCause::Init, Some("Toml feature is not enabled".to_string())))
            },
        }

        Ok(())
    }

    fn init_by_type_from_path<T: DeserializeOwned>(config_type: ConfigType, path: &str) -> ConfigResult<T> {
        match config_type {
            ConfigType::Json => {
                let config = std::fs::read_to_string(path)
                    .map_err(| error| ConfigError::new(ErrorCause::FileOpen, Some(error.to_string())))?;

                serde_json::from_str::<T>(&config)
                    .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
            },
            ConfigType::Yaml => {
                unimplemented!()
            },
            ConfigType::Toml => {
                unimplemented!()
            },
        }
    }

    fn init_by_type_from_str<T: DeserializeOwned>(config_type: ConfigType, input: &str) -> ConfigResult<T> {
        match config_type {
            ConfigType::Json => {
                serde_json::from_str::<T>(&input)
                    .map_err(|error| ConfigError::new(ErrorCause::Deserializing, Some(error.to_string())))
            },
            ConfigType::Yaml => {
                unimplemented!()
            },
            ConfigType::Toml => {
                unimplemented!()
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
