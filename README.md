# Mentat Bitcoin Rosetta Implementation

## Running

##### Installing from Source

After cloning this repository, run:

```text
make build-local
```

#### Run Docker

Running these commands will start a Docker container in [detached mode](https://docs.docker.com/engine/reference/run/#detached--d) with a data directory at `<working directory>/bitcoin-data` and the Rosetta API accessible at port `8080`.

##### Configuration

the file [docker.conf.toml](docker.conf.toml) contains the settings that the docker image will use

##### Command Examples

You can run these commands from the command line. If you cloned the repository, you can use the `make` commands shown after the examples.

###### **Mainnet:Online**

Uncloned repo:
```text
docker run -d --rm --ulimit "nofile=100000:100000" -v "$(pwd)/bitcoin-data:/data" -p 8080:8080 -p 8333:8333 rosetta-bitcoin:latest
```
Cloned repo:
```text
make run-mainnet-online
```

###### **Mainnet:Offline**

Uncloned repo:
```text
docker run -d --rm -p 8081:8081 rosetta-bitcoin:latest
```
Cloned repo:
```text
make run-mainnet-offline
```

###### **Testnet:Online**

Uncloned repo:
```text
docker run -d --rm --ulimit "nofile=100000:100000" -v "$(pwd)/bitcoin-data:/data" -p 8080:8080 -p 18333:18333 rosetta-bitcoin:latest
```

Cloned repo: 
```text
make run-testnet-online
```

###### **Testnet:Offline**

Uncloned repo:
```text
docker run -d --rm -p 8081:8081 rosetta-bitcoin:latest
```

Cloned repo: 
```text
make run-testnet-offline
```

### Example Requests

We supply an Insomnia file with example requests for every endpoint. You can find the file [here](tools/Insomnia_example_payloads.json), or if you already have Insomnia installed you can use the button below to automatically import them.

<!-- the url being linked here is `insomnia://app/import?uri=https://github.com/monadicus/rosetta-bitcoin/blob/main/tools/Insomnia_example_payloads.json`. i was forced to use a link shortener because github refuses to directly link non-web uri's for "security reasons". `https://github.com/github/markup/issues/933#issuecomment-355426548` -->
[![Run in Insomnia](https://insomnia.rest/images/run.svg)](https://tinyurl.com/sync-btc)

### Regtest Node

Running this regtest bitcoin example requires `docker` and uses docker for the running the node.

This node is setup for you and works out of the box.

Start the rosetta-bitcoin with: `cargo run -- regtest.toml`

While this is running, you can generate a "wallet_1" with `./regtest/node_bootstrap.sh`
