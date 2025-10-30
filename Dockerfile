FROM rust:1.90.0 as builder
WORKDIR /usr/src/discordbot
COPY . .
RUN cargo install --path .

FROM debian:trixie-slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/discordbot /usr/local/bin/discordbot
COPY .env .
CMD ["discordbot"]
