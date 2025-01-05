# builder
FROM rust:latest AS builder

WORKDIR /root/builder

RUN rustup target add x86_64-unknown-linux-musl

COPY ./ /root/builder

RUN cargo build --release --target x86_64-unknown-linux-musl

# Runner
FROM alpine:latest

COPY --from=builder /root/builder/target/x86_64-unknown-linux-musl/release/user-service /usr/bin/user-service

WORKDIR /usr/bin

ENTRYPOINT ["user-service"]