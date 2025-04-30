# -----------------------------------------------------------------------------
# 1. Initial build stage that only builds dependencies
# -----------------------------------------------------------------------------
FROM rustlang/rust:nightly-alpine AS deps-builder

# Install only the essential build dependencies
RUN apk update && \
    apk add --no-cache bash curl npm libc-dev pkgconfig

# Install SASS for styling
RUN npm install -g sass

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# First, only copy the files needed to determine dependencies
COPY Cargo.toml Cargo.lock* ./

# Create required directories
RUN mkdir -p src public

# Create dummy source files to build dependencies
RUN echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Create a dummy file in public directory to ensure it exists
RUN touch public/.keep

# Build dependencies - this layer will be cached unless Cargo.toml/Cargo.lock changes
RUN cargo leptos build --release -vv || true

# Remove the dummy source files and compiled artifacts but keep the dependencies
RUN rm -rf src public/* target/server target/front

# -----------------------------------------------------------------------------
# 2. Main builder stage that uses the cached dependencies
# -----------------------------------------------------------------------------
FROM deps-builder AS builder

# Now copy all source files
COPY . .

# Ensure public directory exists (create if not present in your project)
RUN mkdir -p public

# Build the actual application
RUN cargo leptos build --release -vv

# -----------------------------------------------------------------------------
# 3. Final runner stage with minimal dependencies
# -----------------------------------------------------------------------------
FROM alpine:latest AS runner

# Install only the minimal runtime dependencies
RUN apk update && \
    apk add --no-cache ca-certificates

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
