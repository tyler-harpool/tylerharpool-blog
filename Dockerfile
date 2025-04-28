# syntax=docker/dockerfile:1.4
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
COPY rust-toolchain.toml Cargo.toml Cargo.lock ./

FROM chef AS planner
COPY rust-toolchain.toml .
COPY Cargo.toml Cargo.lock ./
COPY src/lib.rs src/lib.rs
COPY src/main.rs src/main.rs
RUN cargo chef prepare --recipe-path recipe.json --bin tylerharpool-blog

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json ./recipe.json

# cache & compile deps
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --recipe-path recipe.json

# copy your source
COPY . .

# build your binary
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --bin tylerharpool-blog

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/tylerharpool-blog  /app/
COPY --from=builder /app/content/blog /app/content/blog

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080


ENTRYPOINT ["/usr/local/bin/tylerharpool-blog"]
