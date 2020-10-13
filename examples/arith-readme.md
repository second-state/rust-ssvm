# Deepsea arith example

This example shows how to run [DeepSEA](https://github.com/certikfoundation/deepsea) language on rust-ssvm

### Compile arith.ds with DeepSEA dsc

Notice that `getBlockCoinbase` and `returnDataSize` should be deleted in the generated wat file since these EEI are incompatible.

```
dsc arith.ds ewasm > arith.wat
wat2wasm arith.wat
```

### Run arith.wasm with rust-ssvm

you could test `add` `sub` `mul` `div` function and observe their output.

```
cargo run -v --example deepsea_arith \
	example/arith.wasm \
	add 3 5
```

