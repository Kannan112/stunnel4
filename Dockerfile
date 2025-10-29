FROM ubuntu:24.04

# Install stunnel and required dependencies
RUN apt-get update && apt-get install -y \
    stunnel4 \
    ca-certificates \
    net-tools \
    && mkdir -p /etc/stunnel/log/stunnel4 \
    && rm -rf /var/lib/apt/lists/*

# Copy pre-built Rust binary
# Build it first with: cargo build --release
COPY ./target/release/stunnel-space /usr/local/bin/stunnel-manager

# Copy stunnel configuration
COPY ./stunnel.conf /etc/stunnel/

# Make server executable
RUN chmod +x /usr/local/bin/stunnel-manager

# Create working directory for configs
WORKDIR /app

RUN chmod 600 /etc/stunnel/stunnel.pem
RUN chmod 600 /etc/stunnel/server.crt

# Expose gRPC port (50055) and stunnel service ports (50000-50010)
EXPOSE 50055 50000-50010

# Run stunnel and Rust gRPC server together
CMD stunnel4 /etc/stunnel/stunnel.conf & \
    /usr/local/bin/stunnel-manager