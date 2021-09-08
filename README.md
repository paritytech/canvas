# canvas-node

This is a node implementation for Canvas, a [Substrate](https://github.com/paritytech/substrate)
parachain for smart contracts.

It uses Substrate's smart contract module ‒ the
[`contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
pallet.

**Note:** This used to be a standalone smart contract node used for the ink! workshop. We
have moved the standalone node to [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node/).

## Quickstart

### Installation 

You need to have executables of both Polkadot, as well as `canvas-node`.

Binary releases can be found here:
[Polkadot releases](https://github.com/paritytech/polkadot/releases),
[Canvas releases](https://github.com/paritytech/canvas-node/releases).

Alternatively you can install (or build) the projects yourself:

* `cargo install polkadot-node --git https://github.com/paritytech/polkadot.git --force --locked`
* `cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --force --locked`

The `--locked` flag makes the installation (or build) use the same versions as the `Cargo.lock`
in those repositories ‒ ensuring that the last known-to-work version of the dependencies is used.

### Launching

Starting this project is best done using the [`polkadot-launch`](https://github.com/paritytech/polkadot-launch) tool ‒
it starts Polkadot and registers the Canvas parachain on it automatically.

1. Install it: `yarn global add polkadot-launch` or `npm i polkadot-launch -g`.
1. Check that the paths in `polkadot-launch/config.json` point to the `polkadot` and `canvas` executables.
1. `polkadot-launch polkadot-launch/config.json`

## Building from source

Follow the [official installation steps](https://substrate.dev/docs/en/knowledgebase/getting-started/) 
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
git clone https://github.com/paritytech/canvas-node.git 
cd canvas-node/
cargo build --release --locked 
```

### Substrate Compatibility

The latest confirmed working Substrate commit which will then be used is
[6080e7a33e63558bf619c240b1ada2cb08c8b443](https://github.com/paritytech/substrate/commit/6080e7a33e63558bf619c240b1ada2cb08c8b443).

It tracks a modified `polkadot-v0.9.9` branches across:
[`substrate`](https://github.com/paritytech/substrate/tree/hc-contract-experiment-patched-v0.9.9),
[`grandpa-bridge-gadget`](https://github.com/paritytech/grandpa-bridge-gadget/tree/hc-contract-experiment-patched-v0.9.9),
[`polkadot`](https://github.com/paritytech/polkadot/tree/hc-contract-experiment-patched-v0.9.9) and 
[`cumulus`](https://github.com/paritytech/cumulus/tree/hc-contract-experiment-patched-v0.9.9).

### Unstable Features

If you're the type of person that likes to drink your soup before it cools, you might
also be wondering about how to activate unstable `pallet-contracts` features. To do this
you can run the previous installation command with the following flag: 
`--features contracts-unstable-interface`.
