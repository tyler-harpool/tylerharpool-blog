# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# FROM bufbuild/buf AS buf
FROM dart:stable AS dart_sass

# Install Git (needed to clone the repo)
RUN apt-get update && apt-get install -y git

# Clone and setup dart-sass
WORKDIR /dart-sass
RUN git clone https://github.com/sass/dart-sass.git . && \
    dart pub get && \
    dart run grinder protobuf

# Copy your app's SCSS files
COPY style /app/style

# Compile SCSS to CSS
# Adjust the paths to match your project structure
RUN dart ./bin/sass.dart /app/style/main.scss /app/style/output.css --style=compressed

# Continue with the builder stage
FROM builder as final_builder

# Copy the compiled CSS from the dart_sass stage
COPY --from=dart_sass /app/style/output.css /app/style/output.css

# Build the app (now with compiled CSS)
RUN cargo leptos build --release -vv

# Runner stage
FROM rustlang/rust:nightly-bullseye as runner

# Copy the server binary to the /app directory
COPY --from=final_builder /app/target/release/tylerharpool-blog /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=final_builder /app/target/site /app/site
# Copy Cargo.toml if it's needed at runtime
COPY --from=final_builder /app/Cargo.toml /app/
COPY --from=final_builder /app/content/blog /app/content/blog

WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/bin/bash", "-c", "/app/tylerharpool-blog"]
