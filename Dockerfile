FROM lukemathwalker/cargo-chef:0.1.77-rust-alpine3.23 AS chef
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig curl
WORKDIR /app

FROM chef AS planner
COPY ./ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY ./ ./
COPY migrations ./migrations
ENV SQLX_OFFLINE=true
RUN cargo build --release --locked --bin api

FROM alpine:3.23 AS runtime
RUN apk add --no-cache ca-certificates && \
    addgroup -S app && \
    adduser -S app -G app
WORKDIR /app
COPY --from=builder /app/target/release/api ./
USER app
EXPOSE 3000
ENTRYPOINT ["./api"]
