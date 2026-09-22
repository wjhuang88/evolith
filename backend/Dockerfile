FROM rust:1.90-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY third_party ./third_party

RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release 2>/dev/null || true && \
    rm -rf src

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/evolith /app/evolith
COPY migrations/sqlite ./migrations/sqlite
COPY migrations/postgres ./migrations/postgres

RUN useradd --uid 1000 --create-home --shell /usr/sbin/nologin evolith && \
    mkdir -p /var/lib/evolith/git && \
    chown -R evolith:evolith /app /var/lib/evolith

USER evolith

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -sf http://localhost:8080/health/ready || exit 1

CMD ["./evolith"]
