FROM rust:1-bookworm AS builder

WORKDIR /app/backend

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend

COPY --from=builder /app/backend/target/release/backend /usr/local/bin/smit-codes-backend
COPY backend/content ./content

ENV SERVER_ADDR=0.0.0.0:3001
ENV ANALYTICS_DB_URL=sqlite:./db/analytics.db
ENV RATE_LIMIT_TRUST_PROXY_HEADERS=true

EXPOSE 3001
VOLUME ["/app/backend/db"]

CMD ["smit-codes-backend"]
