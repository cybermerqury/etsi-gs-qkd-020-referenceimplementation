FROM ubuntu:24.04

# Create certificates folder
RUN mkdir -p /usr/certs

WORKDIR /bin/
COPY target/release/etsi-020-ref-impl ./

ENTRYPOINT [ "/bin/etsi-020-ref-impl" ]
