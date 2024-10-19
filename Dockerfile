FROM debian:bookworm-slim
COPY /target/x86_64-unknown-linux-gnu/release/external-dns-dynu-webhook /opt/external-dns-dynu-webhook
WORKDIR /opt

ENV RUST_BACKTRACE=full

ENTRYPOINT ["/opt/external-dns-dynu-webhook"]

EXPOSE 80