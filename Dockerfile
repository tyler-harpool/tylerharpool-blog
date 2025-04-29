# Build stage
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu:edge as builder

# Install cargo-leptos
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos

WORKDIR /app
COPY . .

# Set environment variables
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"

# Build the project
RUN cargo leptos build --release --bin tylerharpool-blog --vv

# Final slim runtime
FROM debian:bullseye-slim as runner

# Install SSL certs (needed for HTTPS)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only necessary artifacts
COPY --from=builder /app/public /app/public
COPY --from=builder /app/content /app/content
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/tylerharpool-blog /app/tylerharpool-blog

# Environment configs
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 3000

CMD ["/app/tylerharpool-blog"]
