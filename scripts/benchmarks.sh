#!/bin/bash

steps=50
repeat=20
canvasOutput=./runtime/src/weights
canvasChain=canvas-dev
pallets=(
	pallet_balances
	pallet_collator_selection
	pallet_contracts
	pallet_session
	pallet_timestamp
)

for p in ${pallets[@]}
do
	./target/release/canvas benchmark \
		--chain=$canvasChain \
		--execution=wasm \
		--wasm-execution=compiled \
		--pallet=$p  \
		--extrinsic='*' \
		--steps=$steps  \
		--repeat=$repeat \
		--raw  \
		--output=$canvasOutput
done
