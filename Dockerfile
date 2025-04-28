FROM lukemathwalker/cargo-chef:nightly AS chef
WORKDIR /app

# Make sure rust-toolchain.toml is copied early
COPY rust-toolchain.toml .
COPY Cargo.toml .
COPY Cargo.lock .
COPY . .

FROM chef AS planner
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin tylerharpool-blog

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/tylerharpool-blog /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tylerharpool-blog"]
