FROM rust:latest

COPY . .
RUN cargo build --release
EXPOSE 8090

CMD ./target/release/server