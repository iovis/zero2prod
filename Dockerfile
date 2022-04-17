# docker build --tag zero2prod --file Dockerfile .

# Build stage
# - Just used for compiling, not part of the final build
FROM rust:1.60.0-slim AS builder
WORKDIR /app
RUN apt update && apt install -y lld clang pkg-config libssl-dev
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./zero2prod" ]
