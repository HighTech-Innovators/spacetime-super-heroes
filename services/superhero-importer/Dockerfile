FROM rust:1.91.1
WORKDIR /app
COPY . .
RUN cargo build --bins --release
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/app/target/release/superhero-importer"]
