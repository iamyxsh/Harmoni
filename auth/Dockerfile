# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and the src directory containing your code
COPY Cargo.toml .
COPY src src

# Build the Rust application
RUN cargo build --release

# Expose the port your Rust service will listen on
EXPOSE 5000

# Command to run your Rust service
CMD ["cargo", "run", "--release"]
