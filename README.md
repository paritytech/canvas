# canvas-node

Node implementation for Canvas, a Substrate chain for smart contracts.

## Note

The master branch is currently tracking substrate master in order to include various fixes. Therefore it may not
 build if there are breaking changes.

If it fails to build/install, use the cargo `--locked` flag to ensures that the most recent working version of
 substrate will be used.

Latest commit confirmed working: https://github.com/paritytech/substrate/tree/407cd3af06b03201a506b645c9ee587e5d4748bb

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
