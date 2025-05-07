use crate::{
    error::{ConfigError, ErrorCause},
    types::ConfigResult,
};
use notify::Watcher;
use serde::de::DeserializeOwned;

mod hot_reload;

#[derive(Default, Debug, Clone)]
pub struct ConfigManager;

#[derive(Debug, Clone)]
pub struct ConfigManagerInit<'a, T: DeserializeOwned> {
/// Represents the path where the config file is located.
    /// For consistency in the runtime, it is recommended to use the absolute path to the desired config file.
    ///
    /// This path is optional because the Config Manager can be initialized with either a String with the config loaded,
    /// in which case this field will contain a None, or with the path to the file, in which case ConfigManager will
    /// attempt to open said file and read its content. Only when having the path to the file, this crate is able
    /// to implement Hot Reload, otherwise, HR initialization will return an error
    path: Option<&'a std::path::Path>,
    configuration: Option<T>
}

impl<T: DeserializeOwned> Default for ConfigManagerInit<'_, T> {
    fn default() -> Self {
        ConfigManagerInit {
            path: None,
            configuration: None            
        }
    }
}


pub enum ConfigType {
    Json,
    Yaml,
    Toml,
}

impl ConfigManager {
    pub fn init<'a, T: DeserializeOwned>() -> ConfigManagerInit<'a, T> {
        ConfigManagerInit::<T>::default()
    }
}

impl<'a, T: DeserializeOwned> ConfigManagerInit<'a, T> {
    pub fn configure_from_path(
        mut self,
        config_type: ConfigType,
        path: &'a std::path::Path,
    ) -> ConfigResult<Self> {
        self.add_path(path);
        self.init_by_type_from_path(config_type, path)?;
        Ok(self)
    }
    
    fn add_path(&mut self, path: &'a std::path::Path) {
        self.path = Some(path)
    }

    pub fn configure_from_str(
        mut self,
        config_type: ConfigType,
        input: &str,
    ) -> ConfigResult<Self> {
        self.init_by_type_from_str(config_type, input)?;
        Ok(self)
    }

    fn init_by_type_from_path(
        &mut self,
        config_type: ConfigType,
        path: &std::path::Path,
    ) -> ConfigResult<()> {
        let configuration = match config_type {
            ConfigType::Json => {
                #[cfg(not(feature = "json"))]
                return Err(ConfigError::new(
                    ErrorCause::Init,
                    Some("Json feature is not enabled".to_string()),
                ));
                #[cfg(feature = "json")]
                {
                    let config = std::fs::read_to_string(path).map_err(|error| {
                        ConfigError::new(ErrorCause::FileOpen, Some(error.to_string()))
                    })?;
                    serde_json::from_str::<T>(&config).map_err(|error| {
                        ConfigError::new(ErrorCause::Deserializing, Some(error.to_string()))
                    })
                }
            }
            ConfigType::Yaml => {
                #[cfg(not(feature = "yaml"))]
                return Err(ConfigError::new(
                    ErrorCause::Init,
                    Some("Yaml feature is not enabled".to_string()),
                ));
                #[cfg(feature = "yaml")]
                {
                    let config = std::fs::read_to_string(path).map_err(|error| {
                        ConfigError::new(ErrorCause::FileOpen, Some(error.to_string()))
                    })?;
                    serde_yaml::from_str::<T>(&config).map_err(|error| {
                        ConfigError::new(ErrorCause::Deserializing, Some(error.to_string()))
                    })
                }
            }
            ConfigType::Toml => {
                #[cfg(not(feature = "toml"))]
                return Err(ConfigError::new(
                    ErrorCause::Init,
                    Some("Toml feature is not enabled".to_string()),
                ));
                #[cfg(feature = "toml")]
                {
                    let config = std::fs::read_to_string(path).map_err(|error| {
                        ConfigError::new(ErrorCause::FileOpen, Some(error.to_string()))
                    })?;
                    toml::from_str::<T>(&config).map_err(|error| {
                        ConfigError::new(ErrorCause::Deserializing, Some(error.to_string()))
                    })
                }
            }
        }?;

        self.configuration = Some(configuration);
        Ok(())
    }

    fn init_by_type_from_str(
        &mut self,
        config_type: ConfigType,
        input: &str,
    ) -> ConfigResult<()> {
        let configuration= match config_type {
            ConfigType::Json => {
                #[cfg(not(feature = "json"))]
                return Err(ConfigError::new(
                    ErrorCause::Init,
                    Some("Json feature is not enabled".to_string()),
                ));
                #[cfg(feature = "json")]
                serde_json::from_str::<T>(input).map_err(|error| {
                    ConfigError::new(ErrorCause::Deserializing, Some(error.to_string()))
                })
            }
            ConfigType::Yaml => {
                #[cfg(not(feature = "yaml"))]
                return Err(ConfigError::new(
                    ErrorCause::Init,
                    Some("Yaml feature is not enabled".to_string()),
                ));
                #[cfg(feature = "yaml")]
                serde_yaml::from_str(input).map_err(|error| {
                    ConfigError::new(ErrorCause::Deserializing, Some(error.to_string()))
                })
            }
            ConfigType::Toml => {
                #[cfg(not(feature = "toml"))]
                return Err(ConfigError::new(
                    ErrorCause::Init,
                    Some("Toml feature is not enabled".to_string()),
                ));
                #[cfg(feature = "toml")]
                toml::from_str(input).map_err(|error| {
                    ConfigError::new(ErrorCause::Deserializing, Some(error.to_string()))
                })
            }
        }?;

        self.configuration = Some(configuration);
        Ok(())
    }

    pub fn with_hot_reload<C>(self, update_callback: C) -> ConfigResult<()>
    where
        C: Fn(Result<T, ConfigError>) + Send + Sync + 'static,
    {
        let Some(path) = &self.path else {
            return Err(ConfigError::new(
                ErrorCause::FilePathNotSet,
                Some(String::from(
                    "Attempted to intialize Hot Reload without a specified file path in intialization",
                )),
            ));
        };

        let event_handler = |notification_result: Result<notify::Event, notify::Error>| {
            println!("This function just got called!");
        };
        let mut watcher =
            notify::RecommendedWatcher::new(event_handler, notify::Config::default()).unwrap();

        if let Err(error) = watcher.watch(std::path::Path::new(path), notify::RecursiveMode::NonRecursive) {
            return Err(ConfigError::new(ErrorCause::FileWatcherInitError, Some(error.to_string())))
        };


        Ok(())
    }

    pub fn with_cron(self) -> ConfigResult<()> {
        unimplemented!()

        //  TODO
    }

    pub fn run(self) -> ConfigResult<T> {
        match self.configuration {
            Some(config) => Ok(config),
            None => Err(ConfigError::new(ErrorCause::Init, Some(String::from("Could not find configuration in instance"))))
        }
    }
}
