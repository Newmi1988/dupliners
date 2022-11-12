FROM rust:1.65 as builder
WORKDIR /usr/src/dupliners
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/dupliners /usr/local/bin/dupliners
CMD ["dupliners"]
