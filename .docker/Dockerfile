FROM rust AS builder

WORKDIR /source
COPY . /source
RUN cargo install --path .

FROM debian:stable-slim

ENV GREENBONE_FEED_KEY_SERVER=0.0.0.0

RUN apt-get update \
  && apt-get install -y curl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/

EXPOSE 3000

HEALTHCHECK CMD curl -f http://127.0.0.1:3000/api/v1/health || exit 1

CMD ["greenbone-feed-key"]
