# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine AS runner
RUN apk update && \
    apk add --no-cache \
    chromium \
    ttf-freefont \
    udev \
    font-noto-emoji\
    tslib

WORKDIR /app

COPY --from=builder /work/target/release/tylerharpool-blog /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /app/content/blog /app/content/blog

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/tylerharpool-blog"]
