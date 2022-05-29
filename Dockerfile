# Build Service Node
FROM debian:buster-slim as bitcoin-node-builder

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app
WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive
RUN apt update \
    && apt install -y wget

ENV ARCH=x86_64
ENV BITCOIN_VERSION=22.0

RUN wget https://bitcoin.org/bin/bitcoin-core-$BITCOIN_VERSION/bitcoin-$BITCOIN_VERSION-$ARCH-linux-gnu.tar.gz \
    && tar -xf bitcoin-$BITCOIN_VERSION-$ARCH-linux-gnu.tar.gz \
    && cp -R bitcoin-$BITCOIN_VERSION/bin/bitcoind /app/node-runner

# Build bitcoin Rosetta Mentat
FROM debian:buster-slim as rosetta-mentat-builder

ARG BRANCH="main"

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app
WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y curl clang git

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN git clone -b $BRANCH https://github.com/monadicus/rosetta-bitcoin.git \
    && cd rosetta-bitcoin \
    && rm -rf .cargo \
    && cargo build --profile release-docker \
    && mv ./target/release-docker/rosetta-bitcoin /app/server \
    && mv ./docker.conf.toml /app/conf.toml

## Build Final Image
FROM debian:buster-slim

ENV ROCKET_ENV "production"

RUN apt-get update && apt-get install -y libssl-dev

RUN mkdir -p /app/data \
    && chown -R nobody:nogroup /app 

WORKDIR /app

# Copy binary from bitcoin-node-builder
COPY --from=bitcoin-node-builder /app/node-runner /app/node-runner

# Copy binary from rosetta-bitcoin-server
COPY --from=rosetta-mentat-builder /app/* /app 

# Set permissions for everything added to /app
RUN chmod -R 755 /app/*

CMD ["/app/server", "./conf.toml"]
