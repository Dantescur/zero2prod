FROM rust:1.80.1
WORKDIR /app
RUN apt-get update && apt-get install ldd clang -y
COPY . .
RUN cargo build --release
ENTRYPOINT [ "./target/release/zero2prod" ]

