FROM rust:1.100.0 AS build
ARG TARGETPLATFORM
WORKDIR /build
RUN apt-get update && \
    apt-get install -y --no-install-recommends binaryen ca-certificates curl tar && \
    rm -rf /var/lib/apt/lists/* && \
    wasm-opt --version
RUN echo "Building for targetplatform: ${TARGETPLATFORM}" && \
    case "${TARGETPLATFORM}" in \
        "linux/amd64") BINARY_URL="https://github.com/clockworklabs/SpacetimeDB/releases/download/v2.15.0/spacetime-x86_64-unknown-linux-gnu.tar.gz" ;; \
        "linux/arm64") BINARY_URL="https://github.com/clockworklabs/SpacetimeDB/releases/download/v2.15.0/spacetime-aarch64-unknown-linux-gnu.tar.gz" ;; \
        *) echo "Unsupported platform: ${TARGETPLATFORM}" && exit 1 ;; \
    esac && \
    echo "Downloading ${BINARY_URL}" && \
    curl -L "$BINARY_URL" -o spacetime.tgz
RUN tar xvfz spacetime.tgz
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN ./spacetimedb-cli build --module-path ./superhero-server/
