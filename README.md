# Canvas ‒ A Smart Contracts Parachain

[![linux][a1]][a2] [![matrix][k1]][k2] [![discord][l1]][l2]

[a1]: https://gitlab.parity.io/parity/canvas/badges/master/pipeline.svg
[a2]: https://gitlab.parity.io/parity/canvas/pipelines?ref=master
[k1]: https://img.shields.io/badge/matrix-chat-brightgreen.svg?style=flat
[k2]: https://riot.im/app/#/room/#ink:matrix.parity.io
[l1]: https://img.shields.io/discord/722223075629727774?style=flat-square&label=discord
[l2]: https://discord.com/invite/wGUDt2p

This is a node implementation for Canvas, a [Substrate](https://github.com/paritytech/substrate)
parachain for smart contracts.

If you have any questions, feel free to talk to us on [Element][k2] or on [Discord][l2]
(in the [`ink_smart-contracts`](https://discord.com/channels/722223075629727774/765280480609828864) channel).

**Note:** This used to be a standalone smart contracts node used for the ink! workshop. We
have moved the standalone node to [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node/).


## Developing Smart Contracts for Canvas

<div align="center">
    <img src="./.images/how-it-works.svg" alt="How it works" height="520" />
</div>

This node is based on the
[`substrate-parachain-template`](https://github.com/substrate-developer-hub/substrate-parachain-template/),
which we configured to use Substrate's smart contracts module ‒ the [`contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
pallet.
This `contracts` pallet takes smart contracts as WebAssembly blobs and defines an API
for everything a smart contract needs (storage access, …).
As long as a programming language compiles to WebAssembly and there exists an implementation
of this API in it, you can write a smart contract for this pallet ‒ and thus for Canvas ‒ in
that language.

This is a list of languages you can currently choose from:

* [Parity's ink!](https://github.com/paritytech/ink) for Rust
* [ask!](https://github.com/patractlabs/ask) for Assembly Script
* The [Solang](https://github.com/hyperledger-labs/solang) compiler for Solidity

There are also different user interfaces and command-line tools you can use to deploy
or interact with contracts:

* [Canvas UI](https://paritytech.github.io/canvas-ui/)
* [polkadot-js](https://polkadot.js.org/apps/)

If you are looking for a quickstart, we can recommend
[ink!'s Guided Tutorial for Beginners](https://substrate.dev/substrate-contracts-workshop/#/0/building-your-contract).

## Rococo Deployment

We currently have a live deployment of the Canvas parachain on Rococo, Polkadot's
parachain testnet. You can access the network through Polkadot JS Apps by [clicking here](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fcanvas-rpc.parity.io#/explorer).

The Canvas parachain uses Rococo's native token (ROC) instead of having its own token.
Due to this you'll need ROC in order to deploy contracts on Canvas.

You can get ROC testnet tokens through the [Rococo Faucet Channel on Element](https://matrix.to/#/#rococo-faucet:matrix.org).
Once you have ROC tokens on Rococo, you'll need to use XCM to teleport them over into the
Canvas parachain.

Right now there's no easy way to do that, so you'll need to craft the XCM yourself using
Polkadot JS Apps. However, below is a screenshot to help guide you.

![ROC-to-CAN-XCM](./.images/roc-to-can-xcm.png)

A few notes:
- To get to where the screenshot was taken, navigate to the "Developer > Extrinsics" tab
  and choose `xcmPallet` as the extrinsic you want to submit.
- If using the Polkadot JS Apps extension, set your network preferences to "Allow use on
  any network".
- Change the `AccountId32` argument to your own address.
- When specifying the `MultiAsset` amount to send (`Fungible: Compact<u128>`), the units
  are in [`planks`](https://support.polkadot.network/support/solutions/articles/65000168663-how-many-planck-are-in-a-dot-).
  This means that to send 1 ROC you need to set this value to 1,000,000,000,000.
- The `feeAssetItem` field is set to `0` to indicate that we want to pay fees in the
  Native token of the network (ROC).

Once you have ROC on Canvas you can deploy a contract as you would normally.

(If you're unsure about that last step, our [guided tutorial](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/) 
will clarify that for you in no time.)

## Run it Locally

### Installation

You need to have executables of both Polkadot and Canvas.

Binary releases can be found here:
[Polkadot releases](https://github.com/paritytech/polkadot/releases),
[Canvas releases](https://github.com/paritytech/canvas/releases).

Alternatively you can install (or build) the projects yourself:

* `cargo install --git https://github.com/paritytech/polkadot.git --force --locked`
* `cargo install --git https://github.com/paritytech/canvas.git --force --locked`

The `--locked` flag makes the installation (or build) use the same versions as the `Cargo.lock`
in those repositories ‒ ensuring that the last known-to-work version of the dependencies is used.

### Launching

Starting this project is best done using the [`polkadot-launch`](https://github.com/paritytech/polkadot-launch) tool ‒
it starts Polkadot and registers the Canvas parachain on it automatically.

To use `polkadot-launch` you must ensure that you're using Node.js `v14.x` or newer. If
you're on macOS, you must also ensure that your machine's firewall is disabled. You can
do this by going to "System Preferences" > "Security & Privacy" > "Firewall" and ensuring
that it's off.

Now that you have the prerequisites:
1. Install it with `yarn global add polkadot-launch` or `npm i polkadot-launch -g`
1. Check that the paths in `polkadot-launch/config.json` point to the `polkadot` and `canvas` executables
1. Launch the network with `polkadot-launch polkadot-launch/config.json`

At this point you should be able to use [Polkadot JS Apps](https://polkadot.js.org/apps/)
to connect to the Polkadot relay chain nodes as well as the Canvas collator.

## Building from source

Follow the [official installation steps](https://substrate.dev/docs/en/knowledgebase/getting-started/)
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
git clone https://github.com/paritytech/canvas.git
cd canvas/
cargo build --release --locked 
```

### Substrate Compatibility

The latest confirmed working Substrate commit which will then be used is
[a9465729e2c5d2ef8d87ac404da27e5e10adde8a](https://github.com/paritytech/substrate/commit/a9465729e2c5d2ef8d87ac404da27e5e10adde8a).

### Unstable Features

If you're the type of person that likes to drink your soup before it cools, you might
also be wondering about how to activate unstable `pallet-contracts` features. To do this
you can run the previous installation command with the following flag: 
`--features contracts-unstable-interface`.
