# Stage 1: Builder
FROM rust:1.86-slim AS builder

# Install only essential build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy build files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for release and strip symbols
RUN cargo build --release --locked && \
    strip /app/target/release/gcm

# Stage 2: Runtime
FROM gcr.io/distroless/cc

# Copy only the binary and necessary certificates
COPY --from=builder /app/target/release/gcm /usr/local/bin/gcm
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Environment variables
ENV OLLAMA_MODEL=gemma3

ENTRYPOINT ["/usr/local/bin/gcm"]