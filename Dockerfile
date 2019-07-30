FROM rust:1.36

WORKDIR /app

COPY . /app

RUN cargo build --release

EXPOSE 3001

CMD ["cargo", "run", "--release"]
