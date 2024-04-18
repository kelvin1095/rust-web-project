FROM rust:1.77.2-alpine

WORKDIR /app

COPY Cargo.toml ./

COPY . .

RUN cargo build

CMD ["cargo", "run"]