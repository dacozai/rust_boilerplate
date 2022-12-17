# step0 get cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1.65-bullseye AS chef
WORKDIR /app

# step1 prepare recipe for chef
FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# step2 build cache
FROM rust:1.65.0 AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# step3 build release
FROM rust:1.65.0 AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# step4 run app
FROM debian:bullseye-slim
RUN apt update && apt upgrade -y && apt install mariadb-server -y
COPY --from=builder /app/target/release/rust_boilerplate /app/rust_boilerplate
COPY --from=builder /app/src/cfg /app/src/cfg
WORKDIR /app
EXPOSE 5001

CMD ["./rust_boilerplate"]