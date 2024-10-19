# build container
FROM rust:1.80.0-slim-bookworm AS builder
RUN apt update && apt install -y librust-openssl-dev libssl-dev
RUN mkdir /app
COPY . /app
RUN cd /app && cargo build --release

# target container
FROM rust:1.80.0-slim-bookworm
RUN mkdir /app
COPY --from=builder /app/target/release/external-dns-dynu-webhook /app/external-dns-dynu-webhook
WORKDIR /app
CMD ["/app/external-dns-dynu-webhook"]
EXPOSE 8888
ENV RUST_LOG="trace"
ENV PORT="8888"