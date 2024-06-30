# Use a Rust base image
FROM rust:latest

# Set build arguments
ARG ENV_EXECUTION
ARG API_KEY
ARG API_SECRET

# Set environment variables during build
ENV ENV_EXECUTION=$ENV_EXECUTION
ENV API_KEY=$API_KEY
ENV API_SECRET=$API_SECRET

# Create a directory for your Rust application
WORKDIR /usr/src/app

# Copy your Rust project files into the container
COPY . .

# Build your Rust application
RUN cargo build --release

# Set the entry point for running tests
CMD ["cargo", "test"]

