FROM rust

WORKDIR /app

COPY Cargo.toml ./

COPY . .

RUN cargo build

EXPOSE 3000

CMD ["cargo", "run"]
