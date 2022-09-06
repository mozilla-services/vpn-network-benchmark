FROM rust:latest

ADD . /app
WORKDIR /app/
RUN cargo build --release

CMD ["./target/release/nbms"]
EXPOSE 8080
