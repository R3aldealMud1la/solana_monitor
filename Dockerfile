# ---------- BUILD STAGE ----------
FROM rust:1.75 AS builder

WORKDIR /app

# Копируем Cargo-файлы отдельно (кеш Docker)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Копируем реальный код
COPY src ./src
RUN cargo build --release

# ---------- RUNTIME STAGE ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
 && apt-get install -y ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/solana_monitor .

EXPOSE 3000

CMD ["./solana_monitor"]
