# Stunnel Space - Zero-Downtime Stunnel Configuration Manager

**A production-ready Rust gRPC manager for Stunnel that enables dynamic SSL/TLS tunnel configuration with zero downtime.**

## Overview

Stunnel Space provides programmatic control over stunnel instances through a modern gRPC API, allowing you to manage SSL/TLS tunnels dynamically without interrupting active connections. Built with Rust for reliability and performance, it's designed for production environments where uptime is critical.

### Key Features

- **🔄 Zero-Downtime Configuration**: Reload stunnel configurations without dropping active connections
- **🚀 Dynamic Provider Management**: Add or remove SSL tunnel providers on-the-fly via gRPC
- **📊 Real-Time Monitoring**: Track active connections and tunnel status
- **🔒 Multi-Protocol Support**: IPv4, IPv6, or custom network configurations
- **⚡ High Performance**: Built with Rust and async I/O for minimal overhead
- **🐳 Container-Ready**: Docker support with multi-arch images (amd64, arm64)
- **🔧 Validation Built-In**: Automatic configuration validation before applying changes

## Use Cases

### 1. **Microservices SSL Termination**
Dynamically manage SSL/TLS termination for microservices without service interruption:
```
Client → Stunnel Space (manages) → Stunnel → Backend Services
```
- Add new services without downtime
- Update SSL certificates on-the-fly
- Monitor connection health in real-time

### 2. **Legacy Application Modernization**
Add SSL/TLS to legacy applications that don't support it natively:
```
Modern Clients (HTTPS) → Stunnel → Legacy App (HTTP)
```
- No code changes to legacy applications
- Centralized SSL certificate management
- Gradual migration path to modern protocols

### 3. **Database Connection Security**
Secure database connections across networks:
```
Application → Stunnel (client mode) → Stunnel (server mode) → Database
```
- Encrypt database traffic without application changes
- Support for MySQL, PostgreSQL, MongoDB, etc.
- Zero-downtime certificate rotation

### 4. **Multi-Tenant SaaS Platforms**
Dynamically provision SSL tunnels for new tenants:
```
Tenant A → Stunnel (port 5001) → Service A
Tenant B → Stunnel (port 5002) → Service B
```
- Programmatic tunnel creation via API
- Isolated connections per tenant
- Real-time provisioning and deprovisioning

### 5. **Cloud Migration & Hybrid Deployments**
Bridge on-premise and cloud infrastructure securely:
```
Cloud Services ←→ Stunnel Space ←→ On-Premise Systems
```
- Secure communication across environments
- Dynamic routing as services migrate
- No downtime during infrastructure changes

### 6. **IoT Device Management**
Manage SSL tunnels for IoT device fleets:
```
IoT Devices → Stunnel (client) → Stunnel Space → Backend
```
- Centralized certificate management
- Dynamic device onboarding/offboarding
- Connection monitoring and health checks

## Why Stunnel Space?

| Challenge | Traditional Approach | Stunnel Space Solution |
|-----------|---------------------|------------------------|
| Configuration changes | Manual file edits + restart | gRPC API calls, zero downtime |
| Adding new services | Service interruption required | Dynamic provider addition |
| Monitoring | Parse log files manually | Real-time gRPC status queries |
| Automation | Shell scripts + SSH | Modern gRPC API integration |
| Validation | Trial and error | Built-in config validation |
| Multi-environment | Separate configs per env | Branch-based configurations (v4/v6/main) |

## Branch Selection

This project supports different network configurations across branches:

- **`main`** - Custom/flexible configuration (recommended for most users)
- **`v4`** - IPv4-only optimized version
- **`v6`** - IPv6-only optimized version

To switch branches:
```bash
# For IPv4 support
git checkout v4

# For IPv6 support
git checkout v6

# For custom/flexible configuration
git checkout main
```

## Project Structure

```
stunnel-space/
├── Cargo.toml           # Rust project configuration
├── Cargo.lock           # Dependency lock file
├── build.rs             # Build script for proto generation
├── Makefile             # Build automation
├── Dockerfile           # Container build configuration
├── README.md            # This file
├── .gitignore           # Git ignore patterns
├── stunnel.conf         # Sample stunnel configuration
├── proto/
│   └── stunnel.proto    # gRPC service definitions
└── src/
    └── main.rs          # Main application code
```

## Generated Files

During build, the proto files are compiled to:
- `target/debug/build/stunnel-space-*/out/stunnel.rs` (debug builds)
- `target/release/build/stunnel-space-*/out/stunnel.rs` (release builds)

These are automatically included via the `tonic::include_proto!` macro.

## Building

### Local Build
```bash
# Debug build
make build
# or
cargo build

# Release build
make release
# or
cargo build --release

# Generate proto files (happens automatically during build)
make protogen
```

### Docker Build
```bash
docker build -t stunnel-manager .
```

## Running

### Local
```bash
# Default configuration
cargo run

# Custom configuration
STUNNEL_CONF_PATH=/path/to/stunnel.conf STUNNEL_PID_FILE=/path/to/stunnel.pid cargo run
```

### Docker
```bash
# Run with default port mappings
docker run -p 50055:50055 -p 50000-50010:50000-50010 stunnel-manager

# Or use the helper function from export.sh
source export.sh
run_docker
```

## Port Configuration

The application exposes the following ports:
- **50055**: gRPC server (configurable via GRPC_PORT)
- **50000-50010**: Stunnel service ports for SSL/TLS tunneling

These ports allow you to configure multiple stunnel services, each listening on a different port in the 50000-50010 range.

## gRPC API

The service listens on port `50055` and provides:

- **ReloadConfig**: Validate and reload stunnel configuration
- **GetStatus**: Check stunnel status and active connections
- **UpdateConfig**: Update configuration with validation
- **GenerateConfig**: Generate new stunnel configuration
- **AddProvider**: Add new service providers to existing config

## Development

### Prerequisites
- Rust 1.73+
- Protocol Buffers compiler (protoc)
- Stunnel (for testing)

### Testing
```bash
make test
```

### Code Formatting
```bash
make fmt
```

### Linting
```bash
make clippy
```

## Environment Variables

The application can be configured using environment variables. You have three options:

### Option 1: Using .env file
```bash
# Copy the example file
cp .env.example .env

# Edit the .env file with your configuration
nano .env

# Run the application (will auto-load .env)
cargo run
```

### Option 2: Using export.sh
```bash
# Source the export script
source export.sh

# View current configuration
print_config

# Run the server
run_server
```

### Option 3: Direct environment variables
```bash
export STUNNEL_CONF_PATH=/path/to/stunnel.conf
export STUNNEL_PID_FILE=/path/to/stunnel.pid
export GRPC_PORT=8080
cargo run
```

### Available Variables

- `STUNNEL_CONF_PATH`: Path to stunnel configuration file (default: `./stunnel.conf`)
- `STUNNEL_PID_FILE`: Path to stunnel PID file (default: `/tmp/stunnel.pid`)
- `GRPC_PORT`: gRPC server port (default: `50055`)
- `LOG_LEVEL`: Log level - debug, info, warn, error (default: `info`)
- `SSL_CERT_DIR`: Path to SSL certificates directory
- `STUNNEL_ACCEPT_PORT`: Default stunnel accept port
- `STUNNEL_CONNECT_HOST`: Default stunnel connect host
- `STUNNEL_CONNECT_PORT`: Default stunnel connect port
- `RUST_LOG`: Rust log configuration (default: `stunnel_space=info`)

See `.env.example` for a complete list of available variables

## License

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.73%2B-orange)
![Docker](https://img.shields.io/badge/Docker-ready-blue)
