FROM rust as builder
WORKDIR /app

# cache dependencies
COPY Cargo.toml Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src /app/src/
RUN touch /app/src/main.rs
RUN cargo build --release


FROM debian:bookworm-slim
RUN apt update && apt install openssl ca-certificates -y
COPY --from=builder /app/target/release/wumpus-webhook /usr/local/bin/wumpus-webhook

EXPOSE 4056
ENTRYPOINT ["/usr/local/bin/wumpus-webhook"]

