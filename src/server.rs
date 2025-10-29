use chrono::Utc;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tonic::{Request, Response, Status};

use crate::stunnel::stunnel_manager_server::StunnelManager;
use crate::stunnel::{
    AddProviderRequest, AddProviderResponse, GenerateConfigRequest, GenerateConfigResponse,
    ReloadRequest, ReloadResponse, RemoveProviderRequest, RemoveProviderResponse, StatusRequest,
    StatusResponse, UpdateConfigRequest, UpdateConfigResponse,
};
use crate::utils::{
    backup_file, get_active_connections, get_stunnel_pid, reload_stunnel, start_stunnel,
    validate_stunnel_config,
};

#[derive(Debug, Clone)]
pub struct StunnelServer {
    config_path: String,
    pid_file: String,
}

impl StunnelServer {
    pub fn new(config_path: String, pid_file: String) -> Self {
        Self {
            config_path,
            pid_file,
        }
    }
}

// Helper: write atomically by writing to a temp file then renaming.
fn atomic_write(path: &str, content: &str) -> io::Result<()> {
    let tmp_path = format!("{}.tmp.{}", path, std::process::id());
    {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(&tmp_path, path)?;
    Ok(())
}

// Helper: best-effort check if a process exists (works on Linux by checking /proc).
fn process_running(pid: i32) -> bool {
    Path::new(&format!("/proc/{}", pid)).exists()
}

#[tonic::async_trait]
impl StunnelManager for StunnelServer {
    async fn reload_config(
        &self,
        request: Request<ReloadRequest>,
    ) -> Result<Response<ReloadResponse>, Status> {
        let req = request.into_inner();
        let config_path = if req.config_path.is_empty() {
            self.config_path.clone()
        } else {
            req.config_path
        };

        // Validate only if requested
        if req.validate_only {
            match validate_stunnel_config(&config_path) {
                Ok(_) => {
                    return Ok(Response::new(ReloadResponse {
                        success: true,
                        message: "Configuration is valid".to_string(),
                        pid: 0,
                    }));
                }
                Err(e) => {
                    return Ok(Response::new(ReloadResponse {
                        success: false,
                        message: format!("Config validation failed: {}", e),
                        pid: 0,
                    }));
                }
            }
        }

        // Try to get existing PID and reload
        match get_stunnel_pid(&self.pid_file) {
            Ok(pid) => {
                // Ensure process is actually running before attempting reload
                if process_running(pid as i32) {
                    // Send SIGHUP to reload configuration
                    match reload_stunnel(pid) {
                        Ok(_) => Ok(Response::new(ReloadResponse {
                            success: true,
                            message: "Configuration reloaded successfully".to_string(),
                            pid,
                        })),
                        Err(e) => Ok(Response::new(ReloadResponse {
                            success: false,
                            message: format!("Failed to reload stunnel: {}", e),
                            pid: 0,
                        })),
                    }
                } else {
                    // PID file exists but process not running - start new instance
                    match start_stunnel(&config_path) {
                        Ok(new_pid) => Ok(Response::new(ReloadResponse {
                            success: true,
                            message: "Stunnel restarted successfully (stale pid)".to_string(),
                            pid: new_pid,
                        })),
                        Err(e) => Ok(Response::new(ReloadResponse {
                            success: false,
                            message: format!("Failed to start stunnel after stale pid: {}", e),
                            pid: 0,
                        })),
                    }
                }
            }
            Err(e) => {
                // Start new stunnel instance
                println!("Starting new stunnel instance: {}", e);
                match start_stunnel(&config_path) {
                    Ok(pid) => Ok(Response::new(ReloadResponse {
                        success: true,
                        message: "Stunnel started successfully".to_string(),
                        pid,
                    })),
                    Err(e) => Ok(Response::new(ReloadResponse {
                        success: false,
                        message: format!("Failed to start stunnel: {}", e),
                        pid: 0,
                    })),
                }
            }
        }
    }

    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        match get_stunnel_pid(&self.pid_file) {
            Ok(pid) => {
                let connections = get_active_connections();
                Ok(Response::new(StatusResponse {
                    is_running: process_running(pid as i32),
                    pid,
                    config_path: self.config_path.clone(),
                    active_connections: connections,
                }))
            }
            Err(_) => Ok(Response::new(StatusResponse {
                is_running: false,
                pid: 0,
                config_path: self.config_path.clone(),
                active_connections: vec![],
            })),
        }
    }

    async fn update_config(
        &self,
        request: Request<UpdateConfigRequest>,
    ) -> Result<Response<UpdateConfigResponse>, Status> {
        let req = request.into_inner();
        let config_path = if req.config_path.is_empty() {
            self.config_path.clone()
        } else {
            req.config_path
        };

        // Backup existing config
        let backup_path = match backup_file(&config_path) {
            Ok(path) => path,
            Err(e) => {
                return Ok(Response::new(UpdateConfigResponse {
                    success: false,
                    message: format!("Failed to backup config: {}", e),
                }));
            }
        };

        // Write new config atomically
        if let Err(e) = atomic_write(&config_path, &req.config_content) {
            // Attempt to restore from backup if write partially failed
            let _ = fs::copy(&backup_path, &config_path);
            return Ok(Response::new(UpdateConfigResponse {
                success: false,
                message: format!("Failed to write config: {}", e),
            }));
        }

        // Validate new config
        if let Err(e) = validate_stunnel_config(&config_path) {
            // Restore backup
            match fs::copy(&backup_path, &config_path) {
                Ok(_) => {
                    return Ok(Response::new(UpdateConfigResponse {
                        success: false,
                        message: format!("Invalid configuration: {}. Restored previous config.", e),
                    }));
                }
                Err(copy_err) => {
                    // Log restoration error and return failure
                    eprintln!(
                        "Failed to restore backup after validation error: {}",
                        copy_err
                    );
                    return Ok(Response::new(UpdateConfigResponse {
                        success: false,
                        message: format!(
                            "Invalid configuration: {}. Failed to restore backup: {}",
                            e, copy_err
                        ),
                    }));
                }
            }
        }

        Ok(Response::new(UpdateConfigResponse {
            success: true,
            message: "Configuration updated successfully".to_string(),
        }))
    }

    async fn generate_config(
        &self,
        request: Request<GenerateConfigRequest>,
    ) -> Result<Response<GenerateConfigResponse>, Status> {
        let req = request.into_inner();
        let mut config_content = String::new();

        // Global settings
        config_content.push_str("; Stunnel configuration generated by Rust gRPC server\n");
        config_content.push_str(&format!("; Generated at: {}\n\n", Utc::now().to_rfc3339()));

        if req.foreground {
            config_content.push_str("foreground = yes\n");
        }

        config_content.push_str("debug = 7\n");

        let pid_file = if !req.pid_file.is_empty() {
            req.pid_file
        } else {
            "/var/run/stunnel.pid".to_string()
        };
        config_content.push_str(&format!("pid = {}\n", pid_file));

        if !req.cert_path.is_empty() {
            config_content.push_str(&format!("cert = {}\n", req.cert_path));
        }
        if !req.key_path.is_empty() {
            config_content.push_str(&format!("key = {}\n", req.key_path));
        }
        if !req.ca_path.is_empty() {
            config_content.push_str(&format!("CAfile = {}\n", req.ca_path));
            config_content.push_str("verify = 2\n");
        }

        config_content.push_str("\n");

        // Add each provider as a service
        for provider in req.providers {
            config_content.push_str(&format!("; {} service\n", provider.name));
            config_content.push_str(&format!("[{}]\n", provider.name));

            if provider.is_client {
                config_content.push_str("client = yes\n");
            }

            config_content.push_str(&format!("accept = {}\n", provider.accept_port));
            config_content.push_str(&format!(
                "connect = {}:{}\n",
                provider.connect_host, provider.connect_port
            ));
            config_content.push_str("\n");
        }

        // Write to file atomically
        if let Err(e) = atomic_write(&self.config_path, &config_content) {
            return Ok(Response::new(GenerateConfigResponse {
                success: false,
                message: format!("Failed to write config file: {}", e),
                config_content: String::new(),
                config_path: String::new(),
            }));
        }

        // Validate the generated config (skip if stunnel not available)
        if let Err(e) = validate_stunnel_config(&self.config_path) {
            println!(
                "Warning: Config validation failed (stunnel may not be installed): {}",
                e
            );
            // Continue anyway - config is generated
        }

        Ok(Response::new(GenerateConfigResponse {
            success: true,
            message: "Configuration generated successfully".to_string(),
            config_content: config_content.clone(),
            config_path: self.config_path.clone(),
        }))
    }

    async fn add_provider(
        &self,
        request: Request<AddProviderRequest>,
    ) -> Result<Response<AddProviderResponse>, Status> {
        let req = request.into_inner();
        let provider = req
            .provider
            .ok_or_else(|| Status::invalid_argument("Provider is required"))?;

        // Read existing config
        let existing_config = match fs::read_to_string(&self.config_path) {
            Ok(content) => content,
            Err(e) => {
                return Ok(Response::new(AddProviderResponse {
                    success: false,
                    message: format!("Failed to read existing config: {}", e),
                    updated_config: String::new(),
                }));
            }
        };

        // Check if provider already exists
        if existing_config.contains(&format!("[{}]", provider.name)) {
            return Ok(Response::new(AddProviderResponse {
                success: false,
                message: format!("Provider {} already exists in config", provider.name),
                updated_config: String::new(),
            }));
        }

        // Add new provider section
        let mut new_section = String::new();
        new_section.push_str(&format!("\n; {} service\n", provider.name));
        new_section.push_str(&format!("[{}]\n", provider.name));

        if provider.is_client {
            new_section.push_str("client = yes\n");
        }

        new_section.push_str(&format!("accept = {}\n", provider.accept_port));
        new_section.push_str(&format!(
            "connect = {}:{}\n",
            provider.connect_host, provider.connect_port
        ));

        // If global cert/CAfile/verify are present in existing config, copy them into the new service
        let mut cert_line: Option<String> = None;
        let mut cafile_line: Option<String> = None;
        let mut verify_line: Option<String> = None;

        for line in existing_config.lines() {
            let trimmed = line.trim();
            if cert_line.is_none() && trimmed.starts_with("cert =") {
                cert_line = Some(trimmed.to_string());
            } else if cafile_line.is_none() && trimmed.starts_with("CAfile =") {
                cafile_line = Some(trimmed.to_string());
            } else if verify_line.is_none() && trimmed.starts_with("verify =") {
                verify_line = Some(trimmed.to_string());
            }
            if cert_line.is_some() && cafile_line.is_some() && verify_line.is_some() {
                break;
            }
        }

        if let Some(line) = cert_line {
            new_section.push_str(&line);
            new_section.push('\n');
        }
        if let Some(line) = cafile_line {
            new_section.push_str(&line);
            new_section.push('\n');
        }
        if let Some(line) = verify_line {
            new_section.push_str(&line);
            new_section.push('\n');
        }

        // Ensure there's exactly one newline between existing content and new section
        let updated_config = if existing_config.ends_with('\n') {
            format!("{}{}", existing_config, new_section)
        } else {
            format!("{}\n{}", existing_config, new_section)
        };

        // Backup and write new config atomically
        if let Err(e) = backup_file(&self.config_path) {
            return Ok(Response::new(AddProviderResponse {
                success: false,
                message: format!("Failed to backup config: {}", e),
                updated_config: String::new(),
            }));
        }

        if let Err(e) = atomic_write(&self.config_path, &updated_config) {
            return Ok(Response::new(AddProviderResponse {
                success: false,
                message: format!("Failed to write updated config: {}", e),
                updated_config: String::new(),
            }));
        }

        // Validate new config (skip if stunnel not available)
        if let Err(e) = validate_stunnel_config(&self.config_path) {
            println!(
                "Warning: Config validation failed (stunnel may not be installed): {}",
                e
            );
            // Continue anyway - config is written
        }

        // Apply immediately if requested
        if req.apply_immediately {
            if let Ok(pid) = get_stunnel_pid(&self.pid_file) {
                // only reload if process exists
                if process_running(pid as i32) {
                    let _ = reload_stunnel(pid);
                }
            }
        }

        Ok(Response::new(AddProviderResponse {
            success: true,
            message: format!("Provider {} added successfully", provider.name),
            updated_config,
        }))
    }

    async fn remove_provider(
        &self,
        request: Request<RemoveProviderRequest>,
    ) -> Result<Response<RemoveProviderResponse>, Status> {
        let req = request.into_inner();
        let name = req.provider_name;

        if name.trim().is_empty() {
            return Ok(Response::new(RemoveProviderResponse {
                success: false,
                message: "provider_name is required".to_string(),
                updated_config: String::new(),
            }));
        }

        // Read existing config
        let existing_config = match fs::read_to_string(&self.config_path) {
            Ok(content) => content,
            Err(e) => {
                return Ok(Response::new(RemoveProviderResponse {
                    success: false,
                    message: format!("Failed to read existing config: {}", e),
                    updated_config: String::new(),
                }));
            }
        };

        if !existing_config.contains(&format!("[{}]", name)) {
            return Ok(Response::new(RemoveProviderResponse {
                success: false,
                message: format!("Provider {} not found in config", name),
                updated_config: existing_config,
            }));
        }

        // Remove the section lines from the config
        let mut result_lines: Vec<String> = Vec::new();
        let lines: Vec<&str> = existing_config.lines().collect();
        let mut i: usize = 0;
        let target_header = format!("[{}]", name);
        let target_comment = format!("; {} service", name);
        let mut skipping = false;

        while i < lines.len() {
            let line = lines[i];
            let trimmed_start = line.trim_start();

            // If line is a pure comment, keep it and skip header detection on it
            if trimmed_start.starts_with(';') {
                // If we're not in skipping mode, preserve comment lines
                if !skipping {
                    result_lines.push(line.to_string());
                }
                i += 1;
                continue;
            }

            let trimmed = line.trim();
            let is_section_header = trimmed.starts_with('[') && trimmed.ends_with(']');

            if !skipping && trimmed == target_header {
                // If previous pushed line is the comment for this service, remove it
                if let Some(last) = result_lines.last() {
                    if last.trim() == target_comment {
                        let _ = result_lines.pop();
                    }
                }
                // Start skipping from this header line
                skipping = true;
                i += 1;
                continue;
            }

            if skipping {
                // Stop skipping when the next section header begins
                if is_section_header {
                    skipping = false;
                    // Do not consume this header here; loop will handle it without skipping
                    continue;
                } else {
                    i += 1;
                    continue;
                }
            }

            result_lines.push(line.to_string());
            i += 1;
        }

        let updated_config = if result_lines.is_empty() {
            String::new()
        } else {
            // Ensure final newline
            let mut s = result_lines.join("\n");
            if !s.ends_with('\n') {
                s.push('\n');
            }
            s
        };

        // Backup and write new config atomically
        if let Err(e) = backup_file(&self.config_path) {
            return Ok(Response::new(RemoveProviderResponse {
                success: false,
                message: format!("Failed to backup config: {}", e),
                updated_config: String::new(),
            }));
        }

        if let Err(e) = atomic_write(&self.config_path, &updated_config) {
            return Ok(Response::new(RemoveProviderResponse {
                success: false,
                message: format!("Failed to write updated config: {}", e),
                updated_config: String::new(),
            }));
        }

        // Validate new config (skip if stunnel not available)
        if let Err(e) = validate_stunnel_config(&self.config_path) {
            println!(
                "Warning: Config validation failed (stunnel may not be installed): {}",
                e
            );
            // Continue anyway - config is written
        }

        // Apply immediately if requested
        if req.apply_immediately {
            if let Ok(pid) = get_stunnel_pid(&self.pid_file) {
                if process_running(pid as i32) {
                    let _ = reload_stunnel(pid);
                }
            }
        }

        Ok(Response::new(RemoveProviderResponse {
            success: true,
            message: format!("Provider {} removed successfully", name),
            updated_config,
        }))
    }
}
