FROM docker.io/library/rust:1.60.0-slim as builder
WORKDIR /usr/src/gmocli
COPY . .
RUN cargo install --path .

FROM docker.io/library/rust:slim-bullseye
COPY --from=builder /usr/local/cargo/bin/gmocli /usr/local/bin/gmocli

ENTRYPOINT ["gmocli"]
