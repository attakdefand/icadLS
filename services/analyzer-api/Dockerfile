# Use Rust official image as the base image
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /usr/src/analyzer-api

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs to build dependencies
RUN mkdir src && echo "fn main() { println!(\"Dummy\"); }" > src/main.rs

# Build dependencies to cache them
RUN cargo build --release

# Remove the dummy main.rs
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Use a minimal image for the runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/analyzer-api/target/release/analyzer-api /usr/local/bin/analyzer-api

# Expose the port the app runs on
EXPOSE 8081

# Run the application
CMD ["analyzer-api"]