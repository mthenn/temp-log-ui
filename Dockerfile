FROM rust:1.67.0 as build

RUN rustup target add wasm32-unknown-unknown

RUN rustup component add clippy

RUN cargo install --locked trunk

WORKDIR /usr/src/temp-log-ui

COPY . .

RUN trunk build --release

RUN cargo clippy

RUN cargo test --verbose

FROM nginx:1.23.3

WORKDIR /usr/share/nginx/html

COPY --from=build /usr/src/temp-log-ui/dist .