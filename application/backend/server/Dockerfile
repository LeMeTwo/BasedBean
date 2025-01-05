FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache libgcc libstdc++ 
WORKDIR /app

COPY --from=builder /app/target/release/server .

RUN chmod +x server

CMD ["./server"]
