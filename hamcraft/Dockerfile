FROM rust:latest

WORKDIR /usr/src/hello_world
COPY . .

RUN cargo install --path .

CMD ["hello_world"]