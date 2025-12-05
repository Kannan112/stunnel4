//! Utility functions for stunnel process management.
//!
//! This module provides helper functions for interacting with stunnel processes,
//! including PID management, configuration validation, connection monitoring,
//! and process lifecycle management.

use crate::stunnel::Connection;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Reads the PID from a file and verifies the process is running.
///
/// # Arguments
///
/// * `pid_file` - Path to the file containing the stunnel process ID
///
/// # Returns
///
/// Returns `Ok(pid)` if the process is running, or an error if the PID file
/// cannot be read or the process is not running.
///
/// # Example
///
/// ```no_run
/// use stunnel_space::utils::get_stunnel_pid;
///
/// match get_stunnel_pid("/var/run/stunnel.pid") {
///     Ok(pid) => println!("Stunnel is running with PID: {}", pid),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_stunnel_pid(pid_file: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let pid_content = fs::read_to_string(pid_file)?;
    let pid: i32 = pid_content.trim().parse()?;

    // Check if process is running by sending signal 0
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(_) => Ok(pid),
        Err(_) => Err("Process not running".into()),
    }
}

/// Retrieves active stunnel connections using netstat.
///
/// This function parses the output of `netstat -tnp` to find active TCP
/// connections associated with stunnel processes.
///
/// # Returns
///
/// A vector of `Connection` objects representing active stunnel connections.
/// Returns an empty vector if netstat fails or no connections are found.
///
/// # Note
///
/// This function requires `netstat` to be installed and may require
/// root/sudo privileges to see process information.
pub fn get_active_connections() -> Vec<Connection> {
    let mut connections = Vec::new();

    // Run netstat to get TCP connections
    let output = Command::new("netstat").args(["-tnp"]).output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("stunnel") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 5 {
                    connections.push(Connection {
                        service_name: String::new(),
                        local_address: fields[3].to_string(),
                        remote_address: fields[4].to_string(),
                        bytes_sent: 0,
                        bytes_received: 0,
                    });
                }
            }
        }
    }

    connections
}

/// Validates a stunnel configuration file.
///
/// Runs `stunnel -test` to verify the configuration file is valid before
/// applying changes to avoid breaking a working stunnel instance.
///
/// # Arguments
///
/// * `config_path` - Path to the stunnel configuration file to validate
///
/// # Errors
///
/// Returns an error if the configuration file is invalid or stunnel is not installed.
///
/// # Example
///
/// ```no_run
/// use stunnel_space::utils::validate_stunnel_conf_path;
///
/// match validate_stunnel_conf_path("/etc/stunnel/stunnel.conf") {
///     Ok(()) => println!("Configuration is valid"),
///     Err(e) => eprintln!("Invalid configuration: {}", e),
/// }
/// ```
pub fn validate_stunnel_conf_path(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("stunnel")
        .args(["-fd", "0", "-test", config_path])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Config validation failed: {}", error).into());
    }

    Ok(())
}

/// Creates a backup copy of a file.
///
/// Copies the specified file to `{original_path}.backup` if it exists.
///
/// # Arguments
///
/// * `path` - Path to the file to backup
///
/// # Returns
///
/// Returns the path to the backup file on success.
///
/// # Errors
///
/// Returns an error if the file copy operation fails.
pub fn backup_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let backup_path = format!("{}.backup", path);
    if Path::new(path).exists() {
        fs::copy(path, &backup_path)?;
    }
    Ok(backup_path)
}

/// Sends a SIGHUP signal to reload stunnel configuration.
///
/// This tells a running stunnel process to reload its configuration without
/// interrupting active connections.
///
/// # Arguments
///
/// * `pid` - Process ID of the stunnel instance to reload
///
/// # Errors
///
/// Returns an error if the signal cannot be sent (e.g., process doesn't exist
/// or insufficient permissions).
pub fn reload_stunnel(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    signal::kill(Pid::from_raw(pid), Signal::SIGHUP)?;
    Ok(())
}

/// Starts a new stunnel process with the specified configuration.
///
/// # Arguments
///
/// * `config_path` - Path to the stunnel configuration file to use
///
/// # Returns
///
/// Returns the process ID of the newly started stunnel instance.
///
/// # Errors
///
/// Returns an error if stunnel fails to start or if the stunnel binary
/// is not found in PATH.
pub fn start_stunnel(config_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let child = Command::new("stunnel").arg(config_path).spawn()?;

    Ok(child.id() as i32)
}
