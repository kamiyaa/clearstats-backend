FROM rust:alpine AS builder

WORKDIR /app

ARG DATABASE_URL

COPY . .

# sqlx
# RUN cargo install sqlx-cli
# RUN cargo sqlx prepare --database-url $DATABASE_URL

# build

## required by rav1e for av1 encoding/decoding
RUN apk add nasm

RUN --mount=type=cache,target=/app/target/release/build \
	cargo build --release

FROM scratch AS output
WORKDIR app

COPY --from=builder /app/target/release/clearstats-* /app

ENTRYPOINT ["/app/clearstats-api-labspace"]
