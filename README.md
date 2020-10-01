# canvas-node

Node implementation for Canvas, a Substrate chain for smart contracts.

To be continued...

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

Add custom types in `https://polkadot.js.org/apps/#/settings/developer`
```
{
  "Address": "AccountId",
  "LookupSource": "AccountId"
}
```

**Note: currently not compatible with polkadot.js UI, it's a WIP**
