# Use the official Rust image as the build environment
FROM rust:1.88 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/SpotifyToYoutube_backend ./backend

# Create a non-root user
RUN useradd -r -s /bin/false appuser && chown appuser:appuser ./backend
USER appuser

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
CMD ["./backend"]
