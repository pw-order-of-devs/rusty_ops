FROM rust:1.75 as builder

WORKDIR /usr/src/app

RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY Cargo.toml .
COPY Cargo.lock .
COPY commons/ commons/
COPY domain/ domain/
COPY persist/ persist/
COPY rusty_web/ rusty_web/

WORKDIR /usr/src/app/rusty_web
RUN trunk build --release

FROM nginx:alpine

COPY --from=builder /usr/src/app/rusty_web/dist /usr/share/nginx/html
COPY --from=builder /usr/src/app/rusty_web/nginx.conf /etc/nginx/conf.d/default.conf