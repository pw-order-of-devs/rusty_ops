FROM rust:1.79-alpine as build-stage

RUN apk add --no-cache build-base openssl libressl-dev musl-dev

RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/app/src

COPY Cargo.toml .
COPY Cargo.lock .
COPY auth/ auth/
COPY auth_macro/ auth_macro/
COPY commons/ commons/
COPY domain/ domain/
COPY persist/ persist/
COPY rusty_agent/ rusty_agent/

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.19 as final-stage

COPY --from=build-stage /usr/app/src/target/x86_64-unknown-linux-musl/release/rusty_agent /app/
CMD ["/app/rusty_agent"]
