FROM ubuntu:24.04

RUN apt-get update && apt-get install -y stunnel4 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY ./stunnel.conf /etc/stunnel/stunnel.conf

EXPOSE 50000-50010

CMD ["stunnel4", "/etc/stunnel/stunnel.conf"]
