# canvas-node

Node implementation for Canvas, a Substrate chain for smart contracts.

## Note

The master branch is currently tracking substrate master in order to include various fixes. Therefore it may not
 build if there are breaking changes.

If it fails to build/install, use the cargo `--locked` flag to ensures that the most recent working version of
 substrate will be used.

Latest commit confirmed working: https://github.com/paritytech/substrate/tree/c6ba7933c840cd262cca4b95cfdfa93452e83f84

## Usage

To run local dev node, do

```
cargo run --release -- --dev
```

To run test net 1, do

```
cargo run --release
```

or

```
cargo run --release -- --chain=./res/testnet-1.json
```

## Running as a parachain

An experimental implementation which allows running `canvas-node` as a parachain, tracking the `rococo-v1` 
branches of
[`substrate`](https://github.com/paritytech/substrate/tree/rococo-v1), 
[`polkadot`](https://github.com/paritytech/polkadot/tree/rococo-v1) and 
[`cumulus`](https://github.com/paritytech/cumulus/tree/rococo-v1) is available on our own 
[`rococo-v1`](https://github.com/paritytech/canvas-node/tree/rococo-v1)
branch.
