FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --shell /usr/sbin/nologin fossim

WORKDIR /app

COPY --from=builder /app/target/release/fossim-backend /usr/local/bin/fossim-backend

RUN mkdir -p /app/db && chown -R fossim:fossim /app

USER fossim

ENV PORT=8000

EXPOSE 8000

VOLUME ["/app/db"]

CMD ["/usr/local/bin/fossim-backend"]
