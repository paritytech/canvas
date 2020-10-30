# canvas-node

Node implementation for Canvas, a Substrate chain for smart contracts.

## Note

The master branch is currently tracking substrate master in order to include various fixes. Therefore it may not
 build if there are breaking changes.

If it fails to build/install, use the cargo `--locked` flag to ensures that the most recent working version of
 substrate will be used.

Latest commit confirmed working: https://github.com/paritytech/substrate/tree/cab986549f964a081343336797bb6cf6b3526335

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
