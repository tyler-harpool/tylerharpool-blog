# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-binstall for easier cargo extensions installation
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz && \
    tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz && \
    cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos and add WASM target
RUN cargo binstall cargo-leptos -y && \
    rustup target add wasm32-unknown-unknown

# Install pre-built Dart SASS
RUN apt-get update && apt-get install -y wget && \
    wget https://github.com/sass/dart-sass/releases/download/1.63.6/dart-sass-1.63.6-linux-x64.tar.gz && \
    tar -xvf dart-sass-1.63.6-linux-x64.tar.gz && \
    cp -r dart-sass/* /usr/local/bin/ && \
    rm -rf dart-sass dart-sass-1.63.6-linux-x64.tar.gz && \
    sass --version

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Compile SCSS to CSS (adjust paths to match your project structure)
RUN if [ -d "style" ]; then \
      sass style/main.scss:style/output.css --style=compressed; \
    fi

# Build the app
RUN cargo leptos build --release -vv

# Runner stage
FROM rustlang/rust:nightly-bullseye as runner

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/tylerharpool-blog /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it's needed at runtime
COPY --from=builder /app/Cargo.toml /app/
# Copy content folder
COPY --from=builder /app/content/blog /app/content/blog

WORKDIR /app

# Set any required env variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/bin/bash", "-c", "/app/tylerharpool-blog"]
