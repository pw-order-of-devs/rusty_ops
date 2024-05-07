FROM rust:1.77-alpine as build-stage

RUN apk add --no-cache build-base openssl libressl-dev musl-dev

RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/app/src

COPY Cargo.toml .
COPY Cargo.lock .
COPY commons/ commons/
COPY domain/ domain/
COPY persist/ persist/
COPY rusty_init/ rusty_init/

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.19 as final-stage

COPY --from=build-stage /usr/app/src/target/x86_64-unknown-linux-musl/release/rusty_init /app/
CMD ["/app/rusty_init"]
