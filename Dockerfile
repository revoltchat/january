# Build Stage
FROM ekidd/rust-musl-builder:nightly-2021-02-13 AS builder
WORKDIR /home/rust/src

RUN USER=root cargo new --bin january
WORKDIR /home/rust/src/january
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Bundle Stage
FROM alpine:latest
RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*
COPY --from=builder /home/rust/src/january/target/x86_64-unknown-linux-musl/release/january ./
EXPOSE 3000
CMD ["./january"]
