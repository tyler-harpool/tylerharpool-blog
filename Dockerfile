# ------------ Build Stage ------------
FROM rustlang/rust:nightly-bullseye AS builder

# 1) Install WASM‐std & tools in one go
RUN rustup component add rust-src \
 && rustup target add wasm32-unknown-unknown \
 && cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos \
 && cargo install wasm-bindgen-cli

WORKDIR /app

# 2) Prime the dependency cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
 && cargo build --release || true

# 3) Copy full source and build
COPY . .

ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
ENV LEPTOS_OUTPUT_NAME="tylerharpool-blog"

RUN cargo leptos build --release

# ------------ Runtime Stage ------------
FROM debian:bullseye-slim AS runner

RUN apt-get update && apt-get install -y ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 4) Copy only the build artifacts
COPY --from=builder /app/public  ./public
COPY --from=builder /app/content ./content
COPY --from=builder /app/target/site    ./site
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/tylerharpool-blog ./tylerharpool-blog

# 5) Env vars for production
ENV RUST_LOG="info" \
    LEPTOS_OUTPUT_NAME="tylerharpool-blog" \
    APP_ENVIRONMENT="production" \
    LEPTOS_SITE_ADDR="0.0.0.0:3000" \
    LEPTOS_SITE_ROOT="site"

EXPOSE 3000

CMD ["./tylerharpool-blog"]
`
