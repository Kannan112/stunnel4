//! Configuration management for stunnel-space.
//!
//! This module provides configuration loading and validation from environment variables,
//! supporting both `.env` files and direct environment variable configuration.

use std::env;
use std::error::Error;
use std::fmt;

/// Configuration for the stunnel-space gRPC server.
///
/// This struct holds all configuration values needed to run the server,
/// including paths to stunnel configuration and PID files, as well as
/// gRPC server settings.

#[derive(Debug, Clone)]
pub struct Config {
    pub config_path: String,
    pub pid_file: String,
    pub grpc_host: String,
    pub grpc_port: String,
    pub log_level: String,
}

/// Error type returned when required configuration variables are missing.
///
/// Contains a list of all missing environment variable names.
#[derive(Debug)]
pub struct ConfigError {
    missing_vars: Vec<String>,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Missing required environment variables: {}",
            self.missing_vars.join(", ")
        )
    }
}

impl Error for ConfigError {}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Required Environment Variables
    ///
    /// - `STUNNEL_CONF_PATH`: Path to stunnel configuration file
    /// - `STUNNEL_PID_FILE`: Path to stunnel PID file
    /// - `GRPC_PORT`: gRPC server port
    ///
    /// # Optional Environment Variables
    ///
    /// - `GRPC_HOST`: gRPC server host (default: "0.0.0.0")
    /// - `LOG_LEVEL`: Log level (default: "info")
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if any required variables are missing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use stunnel_space::Config;
    ///
    /// std::env::set_var("STUNNEL_CONF_PATH", "/etc/stunnel/stunnel.conf");
    /// std::env::set_var("STUNNEL_PID_FILE", "/var/run/stunnel.pid");
    /// std::env::set_var("GRPC_PORT", "50055");
    ///
    /// let config = Config::from_env().expect("Failed to load config");
    /// ```
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut missing_vars = Vec::new();

        // Get config path - REQUIRED
        let config_path = match env::var("STUNNEL_CONF_PATH") {
            Ok(path) => path,
            Err(_) => {
                missing_vars.push("STUNNEL_CONF_PATH".to_string());
                String::new()
            }
        };

        // Get PID file path - REQUIRED
        let pid_file = match env::var("STUNNEL_PID_FILE") {
            Ok(path) => path,
            Err(_) => {
                missing_vars.push("STUNNEL_PID_FILE".to_string());
                String::new()
            }
        };

        // Get gRPC port - REQUIRED
        let grpc_port = match env::var("GRPC_PORT") {
            Ok(port) => port,
            Err(_) => {
                missing_vars.push("GRPC_PORT".to_string());
                String::new()
            }
        };

        // Get gRPC host - OPTIONAL with default (bind all interfaces)
        let grpc_host = env::var("GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        // Get log level - OPTIONAL with default
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        // If any required variables are missing, return error
        if !missing_vars.is_empty() {
            return Err(ConfigError { missing_vars });
        }

        Ok(Config {
            config_path,
            pid_file,
            grpc_host,
            grpc_port,
            log_level,
        })
    }

    /// Returns the formatted gRPC server address.
    ///
    /// Combines `grpc_host` and `grpc_port` into a single address string
    /// suitable for binding the gRPC server.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use stunnel_space::Config;
    /// # std::env::set_var("STUNNEL_CONF_PATH", "/etc/stunnel.conf");
    /// # std::env::set_var("STUNNEL_PID_FILE", "/var/run/stunnel.pid");
    /// # std::env::set_var("GRPC_PORT", "50055");
    /// let config = Config::from_env().unwrap();
    /// assert_eq!(config.get_grpc_address(), "0.0.0.0:50055");
    /// ```
    pub fn get_grpc_address(&self) -> String {
        format!("{}:{}", self.grpc_host, self.grpc_port)
    }

    /// Prints the current configuration to stdout.
    ///
    /// Useful for debugging and verifying configuration on startup.
    pub fn print_config(&self) {
        println!("=== Server Configuration ===");
        println!("gRPC Host: {}", self.grpc_host);
        println!("gRPC Port: {}", self.grpc_port);
        println!("Config Path: {}", self.config_path);
        println!("PID File: {}", self.pid_file);
        println!("Log Level: {}", self.log_level);
        println!("===========================");
    }
}
