FROM rust:1.72.0 AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
RUN apt-get autoremove
RUN apt-get update
RUN apt-get -y install cmake

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apt-get autoremove
RUN apt-get update
RUN apt-get -y install cmake
RUN apt-get -y install protobuf-compiler

RUN rustup component add clippy

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo prisma generate --schema=./database/AuthCore/prisma/schema.prisma
RUN cargo build --release --bin authcore

FROM alpine:latest as certs
RUN apk --update add ca-certificates

FROM gcr.io/distroless/cc
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/target/release/authcore /
EXPOSE 58080
EXPOSE 8080
CMD ["./authcore"]