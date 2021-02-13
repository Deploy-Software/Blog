# 1: Build the exe
FROM rust:latest as base

ENV USER=worker

WORKDIR /code/app

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y libssl-dev libcurl4-openssl-dev && \
    rustup toolchain add stable

RUN cargo install wasm-pack
RUN mkdir -p /code/app
RUN mkdir -p /code/server/static

RUN cargo init --lib
COPY app/Cargo.toml /code/app/Cargo.toml
RUN cargo fetch
COPY app/sample.rs /code/app/src/lib.rs
RUN wasm-pack build --target web --out-name wasm --out-dir ../server/static
RUN rm -f /code/app/target/release/deps/yew_ap*

WORKDIR /code/server
RUN cargo init
COPY server/Cargo.toml /code/server/Cargo.toml
RUN cargo fetch
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f /code/server/target/release/deps/serve*

FROM base AS builder
WORKDIR /code/app
RUN rm -r /code/app/src
COPY app/src /code/app/src
RUN wasm-pack build --target web --out-name wasm --out-dir ../server/static

WORKDIR /code/server
RUN rm -r /code/server/src
COPY server/src /code/server/src
COPY server/migrations /code/server/migrations
COPY server/sqlx-data.json /code/server/sqlx-data.json
RUN cargo build --release

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM rust:slim as deploy
RUN mkdir -p /code/server/target/release/
WORKDIR /code/server
COPY --from=builder /code/server/static /code/server/static
COPY --from=builder /code/server/target/release/server /code/server/target/release/server

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y libssl-dev libcurl4-openssl-dev

EXPOSE 8080
ENTRYPOINT ["/code/server/target/release/server"]
