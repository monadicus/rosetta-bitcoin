.PHONY: build buld-local build-release run-mainnet-online run-mainnet-offline run-testnet-online \
	run-testnet-offline tracing fmt test clean

PWD=$(shell pwd)
NOFILE=100000
BRANCH=main
PWD=$(shell pwd)

build:
	docker build -t mentat-rosetta-bitcoin:latest https://github.com/monadicus/rosetta-bitcoin.git

build-local:
	docker build --no-cache -t mentat-rosetta-bitcoin:latest . --build-arg BRANCH=$(BRANCH)

build-release:
	# make sure to always set version with vX.X.X
	docker build -t mentat-rosetta-bitcoin:$(version) .
	docker save mentat-rosetta-bitcoin:$(version) | gzip > mentat-rosetta-bitcoin-$(version).tar.gz;

run:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -v "${PWD}/data:/data" -p 8080:8080 -p 4132:4132 mentat-rosetta-bitcoin:latest

tracing:
	docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

fmt:
	cargo +nightly fmt --all
	cargo clippy --all

test:
	cargo test --all

clean:
	cargo clean