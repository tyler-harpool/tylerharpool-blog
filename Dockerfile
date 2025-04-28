# syntax=docker/dockerfile:1.4

# 1) Use the stable cargo-chef image (it has rustup installed)
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# 2) Copy your toolchain and manifest first (to maximize cache)
COPY rust-toolchain.toml .
COPY Cargo.toml Cargo.lock ./

# 3) Prepare the build-deps recipe
FROM chef AS planner
RUN cargo chef prepare --recipe-path recipe.json

# 4) Build everything
FROM chef AS builder
WORKDIR /app

# bring in the recipe
COPY --from=planner /app/recipe.json ./recipe.json

# ⬇ cache and compile your build-dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --recipe-path recipe.json

# copy your full source
COPY . .

# ⬇ cache and build your application
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin tylerharpool-blog

# 5) Final minimal runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/tylerharpool-blog /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tylerharpool-blog"]
