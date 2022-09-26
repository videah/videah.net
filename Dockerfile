ARG TRUNK_BINARY="https://github.com/thedodd/trunk/releases/download/v0.15.0/trunk-x86_64-unknown-linux-gnu.tar.gz"
ARG TARGETPLATFORM

# Version of caddy to be used for hosting
ARG CADDY_VERSION=2.6.1

# This Dockerfile uses cargo-chef to allow for multi-stage builds.
# By doing it this way we don't need to compile dependencies every single time we want to create an image.

FROM lukemathwalker/cargo-chef:latest-rust-1.64.0 AS chef
WORKDIR app

FROM chef as trunker
# Manually compile if we're running on anything that isn't x86_64 (like my M1 Macbook for instance)
RUN echo $TRUNK_BINARY
RUN echo $TARGETPLATFORM
RUN rustup target add wasm32-unknown-unknown
# We should grab binaries when possible. Gonna do this in the future.
# RUN wget -qO- $TRUNK_BINARY | tar -xzf- && mv trunk $CARGO_HOME/bin
RUN cargo install --locked trunk && cargo install wasm-bindgen-cli


FROM chef AS planner
COPY . .
# Craft the recipe used to check if we rely on cached dependencies.
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
RUN cargo chef cook --target wasm32-unknown-unknown --release --recipe-path recipe.json
COPY . .

FROM chef as builder
COPY . .

# Copy dependencies over
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

# Copy binaries of trunk and potentially wasm-bindgen over
COPY --from=trunker $CARGO_HOME/bin $CARGO_HOME/bin

RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

FROM caddy:${CADDY_VERSION}-builder AS embedder
RUN git clone https://github.com/mholt/caddy-embed.git && cd caddy-embed && git checkout 6bbec9d
WORKDIR caddy-embed
COPY --from=builder /app/dist files
COPY 404.html files

# Build a custom caddy binary with the site's files embedded.
# This is so we can serve the site straight from memory.
RUN xcaddy build --with github.com/mholt/caddy-embed=.

FROM caddy:${CADDY_VERSION}-alpine AS runtime
WORKDIR app
COPY Caddyfile /etc/caddy/Caddyfile
COPY --from=embedder /usr/bin/caddy-embed/caddy /usr/bin/caddy
