# Build with:
# docker build --tag zero2prod --file Dockerfile .
FROM rust:1.60.0

WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo build --release

ENTRYPOINT [ "./target/release/zero2prod" ]
