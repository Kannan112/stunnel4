#!/bin/bash

# Simple development environment exports
# Usage: source export.sh

# Keep stunnel.conf inside the repository to observe changes
export STUNNEL_CONFIG="$(pwd)/stunnel.conf"

# Use /tmp for development PID file
export STUNNEL_PID_FILE="/tmp/stunnel.pid"

# gRPC server host and port for dev
export GRPC_HOST=127.0.0.1

export GRPC_PORT=50055

# App log level for dev
export LOG_LEVEL="debug"

# Certificate paths for stunnel services (used as global defaults in stunnel.conf)
# Update these paths if your certs live elsewhere
export CERT_FILE="/etc/stunnel/certs/server.pem"
export CA_FILE="/etc/stunnel/certs/server.crt"

echo "Dev environment exported:"
echo "  STUNNEL_CONFIG=$STUNNEL_CONFIG"
echo "  STUNNEL_PID_FILE=$STUNNEL_PID_FILE"
echo "  GRPC_HOST=$GRPC_HOST"
echo "  GRPC_PORT=$GRPC_PORT"
echo "  LOG_LEVEL=$LOG_LEVEL"
echo "  CERT_FILE=$CERT_FILE"
echo "  CA_FILE=$CA_FILE"

# Sync global cert/CAfile in stunnel.conf so new services inherit them in dev
if [ -f "$STUNNEL_CONFIG" ]; then
  if grep -q "^cert =" "$STUNNEL_CONFIG"; then
    sed -i "s|^cert = .*|cert = $CERT_FILE|" "$STUNNEL_CONFIG"
  else
    echo "cert = $CERT_FILE" >> "$STUNNEL_CONFIG"
  fi
  if grep -q "^CAfile =" "$STUNNEL_CONFIG"; then
    sed -i "s|^CAfile = .*|CAfile = $CA_FILE|" "$STUNNEL_CONFIG"
  else
    echo "CAfile = $CA_FILE" >> "$STUNNEL_CONFIG"
  fi
  if ! grep -q "^verify =" "$STUNNEL_CONFIG"; then
    echo "verify = 2" >> "$STUNNEL_CONFIG"
  fi
fi