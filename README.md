# canvas-node

This is a node implementation for Canvas, a [Substrate](https://github.com/paritytech/substrate)
chain for smart contracts.

It uses Substrate's smart contract module ‒ the
[`contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
pallet.


## Note

The master branch is currently tracking Substrate master in order to include
various fixes. Therefore, it may not build if there are breaking changes.

## Installation

Follow the [official installation steps](https://substrate.dev/docs/en/knowledgebase/getting-started/) 
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --force
```

If it fails to build/install, add the cargo `--locked` flag. The installation process
will then use the same versions as the `Cargo.lock` in this repository to ensure that the
most recent working version of Substrate will be used.

The latest confirmed working Substrate commit which will then be used is
[2543f6f6d80b374da8a2986a258987f20c0c28c6](https://github.com/paritytech/substrate/tree/2543f6f6d80b374da8a2986a258987f20c0c28c6).

## Usage

To run a local dev node execute
```
canvas --dev --tmp
```
The `--tmp` implies that a new chain will be created each time the command
is executed. If you want to persist chain state across runs leave it away.

To run `testnet-1` execute

```
canvas --chain=./res/testnet-1.json
```

## Running as a parachain

The [`rococo-v1`](https://github.com/paritytech/canvas-node/tree/rococo-v1) branch
contains an experimental implementation which allows running `canvas-node` as a parachain.

It tracks the `rococo-v1` branches of
[`substrate`](https://github.com/paritytech/substrate/tree/rococo-v1), 
[`polkadot`](https://github.com/paritytech/polkadot/tree/rococo-v1) and 
[`cumulus`](https://github.com/paritytech/cumulus/tree/rococo-v1).
