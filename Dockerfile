FROM rust:alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path .

FROM alpine as runtime
COPY --from=builder /usr/local/cargo/bin/protohackers /usr/local/bin
