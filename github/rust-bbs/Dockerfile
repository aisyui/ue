FROM syui/aios

WORKDIR /app
COPY ./src ./src
COPY ./templates ./templates
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
CMD ["/app/target/release/rust-bbs"]
