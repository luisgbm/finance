# syntax=docker/dockerfile:1

# ---- Build stage --------------------------------------------------------------
# Compile a release binary. SQLx uses runtime queries (no compile-time macros),
# so NO database is needed at build time.
FROM rust:1-bookworm AS builder
WORKDIR /app

# Pre-build dependencies in a cached layer: build a stub with only the manifests,
# then drop in the real sources. This keeps `cargo build` from recompiling every
# crate dependency whenever application code changes.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src
# Bump mtimes so cargo recompiles the application crate (deps stay cached), then build.
RUN find src -name '*.rs' -exec touch {} + \
    && cargo build --release

# ---- Runtime stage ------------------------------------------------------------
# The Postgres driver is pure-Rust (rustls, no libpq/OpenSSL), so a slim Debian
# image with just CA certificates is enough.
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/finance /usr/local/bin/finance

# Container-friendly defaults; everything else is supplied via the environment
# (see docker-compose.yml). Listening on 0.0.0.0 is required inside a container.
ENV BIND_ADDR=0.0.0.0 \
    PORT=8000

EXPOSE 8000
CMD ["finance"]
