FROM rust:1.65 as builder
WORKDIR /usr/src/dupliners
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/dupliners /usr/local/bin/dupliners
CMD ["dupliners"]
