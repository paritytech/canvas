# canvas-node

This is a node implementation for Canvas, a [Substrate](https://github.com/paritytech/substrate)
parachain for smart contracts.

It uses Substrate's smart contract module ‒ the
[`contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
pallet.

## Note

Note: This used to be a standalone smart contract node used for the ink! workshop. We
have moved the standalone node to [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node/).

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

### Substrate Compatibility

The latest confirmed working Substrate commit which will then be used is
[6080e7a33e63558bf619c240b1ada2cb08c8b443](https://github.com/paritytech/substrate/commit/6080e7a33e63558bf619c240b1ada2cb08c8b443).

It tracks a modified `polkadot-v0.9.9` branches across:
[`substrate`](https://github.com/paritytech/substrate/tree/cmichi-tmp-experiment),
[`grandpa-bridge-gadget`](https://github.com/paritytech/grandpa-bridge-gadget/tree/hc-contract-experiment-patched-v0.9.9),
[`polkadot`](https://github.com/paritytech/polkadot/tree/hc-contract-experiment-patched-v0.9.9) and 
[`cumulus`](https://github.com/paritytech/cumulus/tree/hc-contract-experiment-patched-v0.9.9).

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

To simplify your life you'll want to use the
[`polkadot-launch`](https://github.com/paritytech/polkadot-launch) tool. More
instructions to come!
