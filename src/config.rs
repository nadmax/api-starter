use std::net::SocketAddr;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Top-level application configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// HTTP server settings.
    pub server: ServerConfig,
    /// Logging settings.
    pub log: LogConfig,
}

/// HTTP server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Address to bind on (default: 0.0.0.0).
    pub host: String,
    /// Port to listen on (default: 3000).
    pub port: u16,
    /// Request timeout in seconds (default: 30).
    pub request_timeout_secs: u64,
}

/// Logging configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    /// `tracing` filter directive, e.g. `"info,axum_template=debug"`.
    pub filter: String,
}

impl AppConfig {
    /// Load configuration from files and environment.
    ///
    /// # Errors
    /// Returns an error if the configuration files are missing or malformed.
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            )
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000_i64)?
            .set_default("server.request_timeout_secs", 30_i64)?
            .set_default("log.filter", "info")?
            .build()?
            .try_deserialize()
    }

    /// Returns a [`SocketAddr`] built from the server config.
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("invalid host/port in config")
    }
}