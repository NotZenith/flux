# Build Stage
FROM rust:1.74-slim AS builder

WORKDIR /app
COPY . .

RUN cargo build --release --manifest-path core/Cargo.toml
RUN cargo build --release --manifest-path cli/Cargo.toml

# Final Stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/flux /usr/local/bin/flux

ENTRYPOINT ["flux"]
