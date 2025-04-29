# Build stage
FROM rustlang/rust:nightly-bullseye as builder

# Install Node.js and essential tools
RUN apt-get update && apt-get install -y \
    wget \
    curl \
    gnupg \
    build-essential

# Install Node.js 16
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs && \
    node --version && \
    npm --version

# Install SASS
RUN npm install -g sass

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli manually
RUN cargo install wasm-bindgen-cli

# Make an /app dir
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Process SASS files if they exist
RUN if [ -d "style" ] && [ -f "style/main.scss" ]; then \
      sass style/main.scss style/output.css --style=compressed --no-source-map; \
    fi

# Build the Rust binary
RUN cargo build --release

# Build the WASM module
RUN cargo build --lib --target wasm32-unknown-unknown --release

# Process the WASM with wasm-bindgen
RUN wasm-bindgen --target web --no-typescript --out-dir target/site/pkg \
    target/wasm32-unknown-unknown/release/tylerharpool_blog.wasm

# Runner stage
FROM rustlang/rust:nightly-bullseye as runner

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/tylerharpool-blog /app/

# Copy the site directory
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it's needed at runtime
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/content/blog /app/content/blog

WORKDIR /app

# Set any required env variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/bin/bash", "-c", "/app/tylerharpool-blog"]
