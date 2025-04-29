# ------------ Build Stage ------------
FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-leptos
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos

# Install wasm target (for Leptos client-side)
RUN rustup target add wasm32-unknown-unknown

# Set workdir
WORKDIR /app

# 1. Copy only dependency files first (to leverage Docker cache)
COPY Cargo.toml Cargo.lock ./

# (Optional: If you have a workspace, copy other Cargo.tomls as well)

# 2. Pre-cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# 3. Now copy the rest of your source code
COPY . .

# Set environment variables needed by cargo-leptos
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"

# 4. Build full app (server + client)
RUN cargo leptos build --release --bin tylerharpool-blog --vv

# ------------ Runtime Stage ------------
FROM debian:bullseye-slim as runner

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy compiled assets
COPY --from=builder /app/public /app/public
COPY --from=builder /app/content /app/content
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/tylerharpool-blog /app/tylerharpool-blog

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

# Expose the application port
EXPOSE 3000

# Start the server binary
CMD ["/app/tylerharpool-blog"]
