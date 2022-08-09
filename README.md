# Mentat Bitcoin Rosetta Implementation

## Running

TODO

### Example Requests

We supply an Insomnia file with example payloads for every endpoint. You can find the file [here](tools/Insomnia_example_payloads.json), or if you already have Insomnia installed you can use the button below to automatically import them.

[![Run in Insomnia}](https://insomnia.rest/images/run.svg)](insomnia://app/import?uri=https://github.com/monadicus/rosetta-bitcoin/blob/main/tools/Insomnia_example_payloads.json)

### Regtest Node

Running this regtest bitcoin example requires `docker` and uses docker for the running the node.

This node is setup for you and works out of the box.

Start the rosetta-bitcoin with: `cargo run -- regtest.toml`

While this is running, you can generate a "wallet_1" with `./regtest/node_bootstrap.sh`
