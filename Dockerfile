# Use the official Rust image as a base
FROM rust:1.86-slim AS builder

# Install required system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a working directory
WORKDIR /app

# Copy only the necessary files for dependency resolution first
COPY Cargo.toml Cargo.lock ./

# Copy the entire project
COPY . .

# Build for release
RUN cargo build --release --locked

# Final stage with minimal image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git \
    ca-certificates \
    curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/gcm /usr/local/bin/gcm

# Set up environment variables for configuration
ENV OLLAMA_MODEL=gemma3

# Default command that accepts stdin
ENTRYPOINT ["gcm"]