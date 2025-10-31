FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    stunnel4 \
    ca-certificates \
    net-tools \
    && mkdir -p /etc/stunnel/log/stunnel4 /etc/stunnel/certs \
    && rm -rf /var/lib/apt/lists/*

# Copy binary and configs
COPY ./target/release/stunnel-space /usr/local/bin/stunnel-manager
COPY ./stunnel.conf /etc/stunnel/

RUN chmod +x /usr/local/bin/stunnel-manager 
    # chmod 600 /etc/stunnel/certs/server.pem && \
    # chmod 644 /etc/stunnel/certs/server.crt

WORKDIR /app

# EXPOSE 50055

CMD ["/usr/local/bin/stunnel-manager"]
