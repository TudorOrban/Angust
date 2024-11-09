FROM rust:1.56 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/chain_optim_backend .
CMD ["./chain_optim_backend"]
