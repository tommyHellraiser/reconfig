use crate::error::ConfigError;

pub(crate) type ConfigResult<T> = Result<T, ConfigError>;
