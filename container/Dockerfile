FROM rust:1.87.0-slim-bullseye

# Install required system packages
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy only Cargo.toml first to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy full source code
COPY . .

# Build the actual app
RUN cargo build --release

CMD ["./target/release/bob"]
