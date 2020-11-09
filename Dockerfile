FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/app

COPY . ./

RUN cargo build --release

ENTRYPOINT ["./target/release/rust-example-api-server"]
