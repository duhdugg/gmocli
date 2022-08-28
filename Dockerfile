FROM docker.io/library/rust:1.60.0-slim as builder
WORKDIR /usr/src/emocli
COPY . .
RUN cargo install --path .

FROM docker.io/library/rust:slim-bullseye
COPY --from=builder /usr/local/cargo/bin/emocli /usr/local/bin/emocli

ENTRYPOINT ["emocli"]
