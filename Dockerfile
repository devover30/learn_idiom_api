    # Build Stage
FROM rust:1.78.0-alpine3.19 AS builder

RUN apk add openssl-dev musl-dev
RUN rustup target add x86_64-unknown-linux-musl
#RUN apt update && apt install -y musl-tools musl-dev
#RUN apt install -y pkg-config libssl-dev

RUN update-ca-certificates

WORKDIR /code
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

# Image Stage
FROM alpine:latest

COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/learn_idiom_v1 /usr/local/bin/app

EXPOSE 11800

WORKDIR /usr/local/bin

CMD ["./app"]

