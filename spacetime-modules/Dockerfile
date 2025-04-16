FROM rust:1.85 AS build
WORKDIR /build
ADD https://github.com/clockworklabs/SpacetimeDB/releases/download/v1.0.1/spacetime-aarch64-unknown-linux-gnu.tar.gz spacetime.tgz
RUN tar xvfz spacetime.tgz
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN ./spacetimedb-cli build --project-path ./superhero-server/

