# Use the official Rust image with Alpine as the base
FROM rust:alpine3.21 as builder

# Set the current working directory inside the Docker image
WORKDIR /usr/src/semver

# Copy everything from the current directory (on your machine) to the Docker image
COPY . .

# Install musl-dev for compiling Rust programs
RUN apk add musl-dev

# Cross-compile the Rust project for the x86_64-unknown-linux-musl target
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Start a new stage for the final image
FROM alpine:3.21

# Create a user that will run the app
RUN adduser -D semver

# Switch to the newly created user
USER semver

# Copy the binary from the builder stage to the final image
COPY --from=builder /usr/src/semver/target/x86_64-unknown-linux-musl/release/semver /usr/local/bin/

# Run the app when the Docker image is started
CMD ["semver"]
