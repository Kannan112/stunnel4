FROM ubuntu:24.04

RUN apt-get update && apt-get install -y stunnel4 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# # Copy certificates into container
# COPY ./cert/server.pem /etc/stunnel/certs/server.pem
# COPY ./cert/server.crt /etc/stunnel/certs/server.crt
COPY ./stunnel.conf /etc/stunnel/stunnel.conf

# # Set secure permissions
# RUN chmod 600 /etc/stunnel/certs/server.pem && \
#     chmod 644 /etc/stunnel/certs/server.crt

EXPOSE 50000-50010

CMD ["stunnel4", "/etc/stunnel/stunnel.conf"]
