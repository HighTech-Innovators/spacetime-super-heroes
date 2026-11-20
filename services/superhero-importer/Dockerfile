FROM rust:1.100.0
WORKDIR /app
COPY . .
RUN cargo build --bins --release
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/app/target/release/superhero-importer"]
