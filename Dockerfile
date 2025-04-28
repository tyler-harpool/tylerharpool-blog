# Use a nightly Rust cargo-chef
FROM lukemathwalker/cargo-chef:nightly AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo +nightly chef cook --recipe-path recipe.json
# Build application
COPY . .
RUN cargo +nightly build --release --bin tylerharpool-blog

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/tylerharpool-blog /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tylerharpool-blog"]
