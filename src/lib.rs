pub mod config;
pub mod server;
pub mod utils;

pub mod stunnel {
    tonic::include_proto!("vfxstunnel");
}

pub use config::Config;
pub use server::StunnelServer;
