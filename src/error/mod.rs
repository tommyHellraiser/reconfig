use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigError {
    cause: ErrorCause,
    description: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ErrorCause {
    Init,
    FileOpen,
    FilePathNotSet,
    Deserializing,
    FileWatcherInitError
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is the content of the error")
    }
}

impl Display for ErrorCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::Init => "Init",
            Self::FileOpen => "File Open",
            Self::FilePathNotSet => "File Path Not Set",
            Self::Deserializing => "Deserializing",
            Self::FileWatcherInitError => "File Watcher Initialization Error",
        };
        write!(f, "{}", content)
    }
}

impl ConfigError {
    pub(crate) fn new(cause: ErrorCause, description: Option<String>) -> Self {
        Self { cause, description }
    }
    pub fn get_cause(&self) -> String {
        self.cause.to_string()
    }
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl std::error::Error for ConfigError {}
