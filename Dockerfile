FROM rust:1.69-bullseye

WORKDIR /usr/src/app

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/rust-users-service"]