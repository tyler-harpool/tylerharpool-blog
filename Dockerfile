FROM rustlang/rust:nightly-bullseye as builder

# Install needed Rust components
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN rustup component add rust-src --toolchain nightly

# Install cargo-leptos
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos

# Create and move into app directory
WORKDIR /app
COPY . .

# Set environment variables
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"

# Build the project
RUN cargo leptos build --release --bin tylerharpool-blog --vv

# Now set up the final runtime container
FROM debian:bullseye-slim as runner

# Install SSL libraries (needed by Rust binaries)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built assets
COPY --from=builder /app/public /app/public
COPY --from=builder /app/content /app/content
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/tylerharpool-blog /app/tylerharpool-blog

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"
ENV APP_ENVIRONMENT="_
