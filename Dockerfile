# problems with getting npm installed for tailwindcss when using rust images, so just make our own ubuntu build image, slightly longer, but it is just for builder
FROM ubuntu:22.04 as chef

# install standard stuff
RUN apt-get update && apt-get install -y curl build-essential clang

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y
RUN cargo binstall cargo-chef -y
RUN cargo binstall sccache -y

# cache ENV
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# install node
ENV NVM_DIR /root/.nvm
RUN mkdir -p $NVM_DIR && curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash && . ~/.bashrc && nvm install --lts
RUN . $NVM_DIR/nvm.sh && ln -s $NVM_DIR/$(nvm current) $NVM_DIR/cur && ln -s $NVM_DIR/versions/node/$(nvm current) $NVM_DIR/versions/node/cur
ENV NODE_PATH $NVM_DIR/cur/lib/node_modules
ENV PATH      $NVM_DIR/versions/node/cur/bin:$PATH


## planner
FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as Cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json

# -- NB: update --package= name from "tylerharpool-blog" to match your app name in Cargo.toml --
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --package=tylerharpool-blog --bin=tylerharpool-blog --target-dir=target/ --recipe-path recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --package=tylerharpool-blog --target-dir=target/front --target=wasm32-unknown-unknown --recipe-path recipe.json

# builder
FROM chef AS builder
WORKDIR /app
# copy app
COPY . .

# copy cache
COPY --from=cacher /app/target /app/target

# Build the app
RUN /bin/bash -c "source $NVM_DIR/nvm.sh \
        && npm install"

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app

# -- NB: update binary name from "tylerharpool-blog" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/tylerharpool-blog /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
# Copy markdown to container
COPY --from=builder /app/content/blog /app/content/blog

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
# -- NB: update binary name from "tylerharpool-blog" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/tylerharpool-blog"]
