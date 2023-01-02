FROM rust:buster AS builder
WORKDIR /app
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim as runner
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/dso-dojo-2023-01 .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 80
CMD ["./dso-dojo-2023-01"]
