FROM rust:latest

WORKDIR /app

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y sqlite3

CMD ["cargo", "run"]
