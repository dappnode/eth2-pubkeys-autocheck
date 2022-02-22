FROM rust:1.58.1-alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

