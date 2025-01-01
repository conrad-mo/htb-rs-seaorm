FROM rust:latest as base
WORKDIR /app

RUN apt-get update && apt-get install lld clang -y
RUN cargo install cargo-chef --locked

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/hack-the-back ./run

EXPOSE 8000

ENTRYPOINT ["./run"]
