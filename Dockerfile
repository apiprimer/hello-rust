FROM rust:1.76 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

# Create non-root user (optional but best practice)
RUN adduser --disabled-password --gecos '' appuser
USER appuser

COPY --from=builder /usr/local/cargo/bin/hello-rust /usr/local/bin/hello-rust

CMD ["hello-rust"]
