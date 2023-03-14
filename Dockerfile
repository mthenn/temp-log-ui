FROM rust:1.68.0 as build

RUN rustup target add wasm32-unknown-unknown

RUN rustup component add clippy

RUN cargo install --locked trunk

RUN cargo install --locked cargo-about

WORKDIR /usr/src/temp-log-ui

COPY . .

RUN trunk build --release

RUN cargo clippy

RUN cargo test --verbose

RUN cargo about generate about.hbs > license.html

FROM nginx:1.23.3

WORKDIR /usr/share/nginx/html

COPY --from=build /usr/src/temp-log-ui/dist .