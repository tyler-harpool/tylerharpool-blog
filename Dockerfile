# -----------------------------------------------------------------------------
# 1. Initial build stage that only builds dependencies
# -----------------------------------------------------------------------------
FROM rustlang/rust:nightly-alpine AS deps-builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen \
    # Fix: Use the correct package name for pkg-config
    pkgconfig openssl-dev

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# First, only copy the files needed to determine dependencies
COPY Cargo.toml Cargo.lock* ./

# Create dummy source files to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies - this layer will be cached unless Cargo.toml/Cargo.lock changes
RUN cargo leptos build --release -vv

# Remove the dummy source files and compiled artifacts
RUN rm -rf src target

# -----------------------------------------------------------------------------
# 2. Main builder stage that uses the cached dependencies
# -----------------------------------------------------------------------------
FROM deps-builder AS builder

# Now copy all source files
COPY . .

# Build the actual application
RUN cargo leptos build --release -vv

# -----------------------------------------------------------------------------
# 3. Final runner stage with minimal dependencies
# -----------------------------------------------------------------------------
FROM rustlang/rust:nightly-alpine AS runner

RUN apk update && \
    apk add --no-cache \
    chromium \
    ttf-freefont \
    udev \
    font-noto-emoji \
    tslib

WORKDIR /app

# Copy only what's needed to run the application
COPY --from=builder /app/target/release/tylerharpool-blog /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/content/blog /app/content/blog

# Set environment variables and expose port
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

# Run the server
CMD ["/app/tylerharpool-blog"]
