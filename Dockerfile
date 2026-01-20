# Stage 1: Chef (Base)
FROM lukemathwalker/cargo-chef:0.1.73-rust-1.92.0 AS chef
WORKDIR /app
# musl-tools is required for static linking on Alpine
RUN apt update && apt install lld clang musl-tools -y
# Add the musl target to rustup
RUN rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies for the MUSL target
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE=true
# Build the actual binary for the MUSL target
RUN cargo build --release --target x86_64-unknown-linux-musl --bin email-newsletter

# Stage 2: Runtime (Alpine)
FROM alpine:3.20 AS runtime
WORKDIR /app

# Alpine uses 'apk' instead of 'apt'
# We install OpenSSL and CA-certificates for HTTPS support
RUN apk add --no-cache openssl ca-certificates

# Copy the binary from the MUSL target folder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/email-newsletter email-newsletter
COPY configuration configuration
ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./email-newsletter"]