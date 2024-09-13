use colored::*;

pub enum ConfigError {
    ReadError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ReadError(message) => write!(f, "{} {}", "# ReadConfigError:".bold().on_red(), message.red()),
        }
    }
}