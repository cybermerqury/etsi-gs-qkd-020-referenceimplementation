FROM ubuntu:24.04

RUN apt update     && \
    apt upgrade -y && \
    apt install -y    \
    libpq-dev         \
    libssl-dev        \
    && rm -rf /var/lib/apt/lists/*

# Create certificates folder
RUN mkdir -p /usr/certs

WORKDIR /bin/
COPY target/release/etsi-020-ref-impl ./

ENTRYPOINT [ "/bin/etsi-020-ref-impl" ]
