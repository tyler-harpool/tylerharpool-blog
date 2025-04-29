# syntax=docker/dockerfile:1.4

############################################################
# 1) CHEF: install toolchain, wasm & build helpers only once
############################################################
FROM rustlang/rust:nightly-bullseye AS chef

# 1.1) Install rust-src (for wasm32 std), wasm target, and tools
RUN rustup component add rust-src \
 && rustup target add wasm32-unknown-unknown \
 && cargo install --locked cargo-chef \
 && cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos \
 && cargo install --locked wasm-bindgen-cli

WORKDIR /app

############################################################
# 2) PLANNER: generate your recipe.json (dependency graph)
############################################################
FROM chef AS planner

# copy manifest & minimal stubs so cargo metadata works
COPY Cargo.toml Cargo.lock ./
# If your Cargo.toml defines a lib and a bin, you can avoid stubbing
# by specifying --bin, but here we’ll stub both targets:
RUN mkdir src \
 && echo "fn main(){}" > src/main.rs \
 && echo "// lib stub"    > src/lib.rs

# now run cargo-chef to analyze your deps
RUN cargo chef prepare --recipe-path recipe.json \
    --bin tylerharpool-blog

############################################################
# 3) BUILDER: use recipe.json to cache-deps & build your app
############################################################
FROM chef AS builder
WORKDIR /app

# 3.1) fetch & compile only dependencies
COPY --from=planner /app/recipe.json ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

# 3.2) bring in your full source
COPY . .

# 3.3) finally build server + client
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu" \
    LEPTOS_OUTPUT_NAME="tylerharpool-blog"
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo leptos build --release

############################################################
# 4) RUNTIME: a tiny image with only your binary + assets
############################################################
FROM debian:bullseye-slim AS runner

# minimal runtime deps
RUN apt-get update \
 && apt-get install -y ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# copy public/site assets
COPY --from=builder /app/public  ./public
COPY --from=builder /app/content ./content
COPY --from=builder /app/target/site ./site

# copy your server binary
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/tylerharpool-blog ./tylerharpool-blog

# production env
ENV RUST_LOG="info" \
    APP_ENVIRONMENT="production" \
    LEPTOS_SITE_ADDR="0.0.0.0:3000" \
    LEPTOS_SITE_ROOT="site"

EXPOSE 3000
CMD ["./tylerharpool-blog"]
