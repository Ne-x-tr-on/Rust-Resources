# Use the official Rust image
FROM rust:1.81

# Set working directory inside container
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first (for caching)
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory (to allow dependencies build)
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (for faster rebuilds)
RUN cargo build --release

# Now copy the rest of your project
COPY . .

# Build the actual app
RUN cargo build --release

# Run the app
CMD ["./target/release/Test_6"]
