# syntax = docker/dockerfile:1.4
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
# Leverage Docker's cache mount for Rust builds
# Copy early to maximize cache usage
COPY rust-toolchain.toml .
COPY Cargo.toml .
COPY Cargo.lock .
COPY . .

FROM chef AS planner
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# ⬇️ cache build dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --recipe-path recipe.json

COPY . .

# ⬇️ cache final build
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin tylerharpool-blog

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/tylerharpool-blog /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tylerharpool-blog"]
