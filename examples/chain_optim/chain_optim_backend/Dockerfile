FROM rust:1.82-bullseye AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/chain_optim_backend .

RUN apt-get update && apt-get install -y \
    default-mysql-client \
    libmariadb-dev \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

CMD ["./chain_optim_backend"]
