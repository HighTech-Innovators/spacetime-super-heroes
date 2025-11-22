FROM rust:1.91.1 AS build
ARG TARGETPLATFORM
WORKDIR /build
RUN TARGETPLATFORM="linux/amd64" && \
    case "${TARGETPLATFORM}" in \
        "linux/amd64") BINARY_URL="https://github.com/clockworklabs/SpacetimeDB/releases/download/v1.8.0/spacetime-x86_64-unknown-linux-gnu.tar.gz" ;; \
        "linux/arm64") BINARY_URL="https://github.com/clockworklabs/SpacetimeDB/releases/download/v1.8.0/spacetime-aarch64-unknown-linux-gnu.tar.gz" ;; \
        *) echo "Unsupported platform: ${TARGETPLATFORM}" && exit 1 ;; \
    esac && \
    echo "Downloading ${BINARY_URL}" && \
    curl -L "$BINARY_URL" -o spacetime.tgz
RUN tar xvfz spacetime.tgz
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN ./spacetimedb-cli build --project-path ./superhero-server/

