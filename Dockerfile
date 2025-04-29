# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install Node.js and essential tools for frontend processing
RUN apt-get update && apt-get install -y \
    wget \
    curl \
    gnupg \
    build-essential

# Install Node.js 16 properly
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs && \
    node --version && \
    npm --version

# Install cargo-binstall
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz && \
    tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz && \
    cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Install SASS
RUN npm install -g sass

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Process SASS files if they exist (before building the app)
RUN if [ -d "style" ] && [ -f "style/main.scss" ]; then \
      sass style/main.scss style/output.css --style=compressed --no-source-map; \
    fi

# Build the app
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye as runner

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/tylerharpool-blog /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it's needed at runtime
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/content/blog /app/content/blog

WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/bin/bash", "-c", "/app/tylerharpool-blog"]
