//! # Stunnel-Space
//!
//! A Rust-based gRPC manager for programmatic control over stunnel instances.
//!
//! This crate provides a gRPC server that allows you to manage stunnel configurations,
//! including reloading configs, checking status, updating configurations, and managing
//! service providers dynamically.
//!
//! ## Features
//!
//! - **Configuration Management**: Validate, update, and generate stunnel configurations
//! - **Dynamic Provider Management**: Add or remove service providers at runtime
//! - **Status Monitoring**: Check stunnel process status and active connections
//! - **gRPC API**: Modern, efficient API for remote stunnel management
//!
//! ## Example
//!
//! ```no_run
//! use stunnel_space::{Config, StunnelServer};
//! use stunnel_space::stunnel::stunnel_manager_server::StunnelManagerServer;
//! use tonic::transport::Server;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::from_env()?;
//!     let addr = config.get_grpc_address().parse()?;
//!     let server = StunnelServer::new(config.config_path, config.pid_file);
//!
//!     Server::builder()
//!         .add_service(StunnelManagerServer::new(server))
//!         .serve(addr)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod server;
pub mod utils;

pub mod stunnel {
    tonic::include_proto!("vfxstunnel");
}

pub use config::Config;
pub use server::StunnelServer;
