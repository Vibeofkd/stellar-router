FROM rust:1.79-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY contracts/ contracts/
RUN cargo build --release
RUN cargo build --target wasm32-unknown-unknown --release

# ── Test stage ────────────────────────────────────────────────────────────────

FROM builder AS test

RUN cargo test --release

# ── WASM artifacts stage ──────────────────────────────────────────────────────

FROM debian:bookworm-slim AS wasm

COPY --from=builder /app/target/wasm32-unknown-unknown/release/*.wasm /wasm/

# ── API server stage ─────────────────────────────────────────────────────────

FROM debian:bookworm-slim AS api-server

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ /usr/local/bin/
COPY --from=builder /app/target/wasm32-unknown-unknown/release/*.wasm /wasm/

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["stellar-router-api"]

# ── Metrics exporter stage ───────────────────────────────────────────────────

FROM debian:bookworm-slim AS metrics

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ /usr/local/bin/

EXPOSE 9090

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:9090/metrics || exit 1

CMD ["stellar-router-metrics"]
