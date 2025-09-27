# Use Debian-based image
FROM python:3.11-slim

# Install Rust (via rustup)
RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && . "$HOME/.cargo/env"

ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy Python requirements
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy Rust Cargo files (to cache dependencies)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() { println!(\"Hello Quant!\"); }" > src/main.rs
RUN cargo build --release

# Copy all source code
COPY . .

# Expose ports (3000 for Rust API, 8000 for FastAPI/Streamlit)
EXPOSE 3000
EXPOSE 8000

# Command options: choose Rust or Python
CMD ["bash"]
