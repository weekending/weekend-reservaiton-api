###############################################################################
# BUILD IMAGE                                                                 #
###############################################################################
FROM rust:1.87 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    cargo build --release \
    && rm -rf src

COPY src /app/src

RUN cargo build --release

###############################################################################
# RUNTIME IMAGE                                                               #
###############################################################################
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

RUN apt-get update \
    && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

COPY --from=builder /app/target/release/weekend-reservaiton-api /usr/local/bin/weekend-reservaiton-api

CMD ["weekend-reservaiton-api"]
