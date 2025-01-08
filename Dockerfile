# Step 1: Build the application
FROM rust:1.83 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Step 2: Create a runtime image with updated glibc
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/exchange_rs .

# Expose the port the app runs on
EXPOSE 3030

# Command to run the application
CMD ["./exchange_rs"]
