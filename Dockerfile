FROM rust:1.27-stretch as builder

RUN mkdir -p /build/src/bin
RUN echo 'fn main() {}' > /build/src/main.rs
COPY Cargo.toml Cargo.lock /build/
WORKDIR /build
RUN cargo build --release --locked
COPY src /build/src
RUN cargo build --release --locked

FROM debian:stretch-slim
RUN apt update && apt install -y libssl1.1 ca-certificates git
COPY --from=builder /build/target/release/idolmap /usr/local/bin/
COPY idolmap-update.sh /usr/local/bin/
