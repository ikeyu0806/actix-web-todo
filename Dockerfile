FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install cargo-watch

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y sqlite3
