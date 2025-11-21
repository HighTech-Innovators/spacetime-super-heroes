FROM rust:1.91.1 AS build
WORKDIR /build
COPY . .
RUN cargo build --bins --locked --release

FROM gcr.io/distroless/cc-debian12 AS superhero-client
COPY --from=build /build/target/release/superhero-client /
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/superhero-client"]

FROM gcr.io/distroless/cc-debian12 AS superhero-importer 
COPY --from=build /build/target/release/superhero-importer /
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/superhero-importer"]
