ARG TRUNK_BINARY="https://github.com/thedodd/trunk/releases/download/v0.15.0/trunk-x86_64-unknown-linux-gnu.tar.gz"
ARG TARGETPLATFORM

# This Dockerfile uses cargo-chef to allow for multi-stage builds.
# By doing it this way we don't need to compile dependencies every single time we want to create an image.

FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 AS chef
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

FROM nginx:alpine AS runtime
WORKDIR app
COPY --from=builder /app/dist /usr/share/nginx/html
