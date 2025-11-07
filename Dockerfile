# ---- Build stage ----
FROM rust:1.83-bullseye AS builder
WORKDIR /app

# Install build deps (pkg-config + OpenSSL headers are handy for tls-native-tls)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# If you *must* keep edition=2024 right now, use nightly:
RUN rustup toolchain install nightly && rustup default nightly

# Cache deps
COPY Cargo.toml Cargo.lock ./
# Create a dummy src to let cargo resolve deps
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release || true

# Now copy real sources
COPY . .
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl1.1 || true && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/shorty /app/shorty
EXPOSE 8080
CMD ["./shorty"]

