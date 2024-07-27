FROM rust:1.80-alpine AS build-stage

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
COPY rusty_init/ rusty_init/
COPY rusty_server/ rusty_server/
COPY tests/ tests/

RUN cargo build --release --target x86_64-unknown-linux-musl \
    --package rusty_init

FROM alpine:3.19 AS final-stage

COPY --from=build-stage /usr/app/src/target/x86_64-unknown-linux-musl/release/rusty_init /app/
COPY --from=build-stage /usr/app/src/rusty_init/sql /app/pg/sql

CMD ["/app/rusty_init"]
