FROM rust:1.55 AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin au-health-backend

FROM debian:buster-slim AS runtime
WORKDIR /app
# RUN apt-get update -y \
#     && apt-get install -y --no-install-recommends openssl \
#     && apt-get autoremove -y \
#     && apt-get clean -y \
#     && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/au-health-backend au-health-backend
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./au-health-backend"]`