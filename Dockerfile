# Build stage
FROM rustlang/rust:nightly-alpine as builder

# Install essential dependencies
RUN apk add --no-cache build-base npm bash curl

# Create working directory
WORKDIR /app

# Copy only project structure files first
COPY Cargo.toml Cargo.lock ./

# Create necessary directories and files
RUN mkdir -p public src
RUN mkdir -p content/blog

# Copy source code and content
COPY src ./src/
COPY content ./content/
COPY style ./style/
COPY public ./public/

# Install tools
RUN npm install -g sass && \
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh && \
    rustup target add wasm32-unknown-unknown

# Temporary fix for the public directory
RUN touch public/.keep

# Build the application
RUN LEPTOS_SITE_ROOT=public cargo leptos build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Set working directory
WORKDIR /app

# Copy binary and static files
COPY --from=builder /app/target/release/tylerharpool-blog /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/content/blog /app/content/blog

# Set environment variables
ENV RUST_LOG="info" \
    LEPTOS_SITE_ADDR="0.0.0.0:8080" \
    LEPTOS_SITE_ROOT="site"

# Expose port
EXPOSE 8080

# Run the server
CMD ["/app/tylerharpool-blog"]
