# Etap kompilacji: Rust
FROM rust:latest AS builder

# Instalacja wymaganych narzędzi kompilacyjnych
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean

WORKDIR /app
COPY . .
RUN cargo build --release

# Etap uruchamiania: obraz Ubuntu z nowszą wersją glibc
FROM ubuntu:latest

# Instalacja wymaganych bibliotek
RUN apt-get update && apt-get install -y \
    libgcc1 \
    libstdc++6 \
    libc6 \
    && apt-get clean

WORKDIR /app

# Kopiowanie skompilowanej aplikacji z etapu kompilacji
COPY --from=builder /app/target/release/server .

# Nadanie praw
RUN chmod +x server

# Uruchomienie aplikacji
CMD ["./server"]
