FROM rust:1.75 as build-stage

RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /usr/app/src

COPY Cargo.toml .
COPY Cargo.lock .
COPY commons/ commons/
COPY domain/ domain/
COPY persist/ persist/
COPY rusty_server/ rusty_server/

RUN cargo build --release

FROM debian:stable-slim as final-stage

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libc-bin libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build-stage /usr/app/src/target/release/rusty_server /app/
CMD ["/app/rusty_server"]
