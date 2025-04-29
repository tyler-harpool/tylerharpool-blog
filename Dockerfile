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


# Download and install nvm:
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
# in lieu of restarting the shell
# Download and install Node.js:
RUN nvm install 22
# Verify the Node.js version:
RUN node -v # Should print "v22.15.0".
RUN nvm current # Should print "v22.15.0".
# Verify npm version:
RUN npm -v # Should print "10.9.2".

# Build the app
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye as runner

# -- NB: update binary name from "tylerharpool-blog" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/tylerharpool-blog /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/content/blog /app/content/blog

# RUN touch /app/.env

WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/bin/bash", "-c", "/app/tylerharpool-blog"]
