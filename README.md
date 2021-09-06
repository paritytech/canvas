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
[ed702e8246d5c4f82e686fb044ac6c2e6cd269cf](https://github.com/paritytech/substrate/tree/ed702e8246d5c4f82e686fb044ac6c2e6cd269cf).

### Unstable Features
If you're the type of person that likes to drink your soup before it cools, you might
also be wondering about how to activate unstable `pallet-contracts` features. To do this
you can run the previous installation command with the following flag: 

`--features contracts-unstable-interface`.

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
