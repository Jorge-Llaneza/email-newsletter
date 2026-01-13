FROM rust:1.92.0
LABEL authors="jorge"

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/email-newsletter"]