# Integrate fe with rust-ssvm

In this example, we would try to run fe tests under `fe/compiler/tests/fixtures`.

## Tools
Build fe and Second State toolchain with the instructions inside the repository

| Name                                                   |  commit   | Usage                        |
|:------------------------------------------------------ |:---:| ---------------------------- |
| [fe](https://github.com/ethereum/fe)                   |  7db1bdc   | compile fe to yul IR         |
| [SOLL](https://github.com/second-state/soll)           |  d349722   | compile yul to ewasm bytecode |
| [rust-ssvm](https://github.com/second-state/rust-ssvm) |  dc7d33a   | execute ewasm bytecode       |

## Steps

### Compile \*.fe files into ewasm bytecode
First, we compile \*.fe files into yul. Fe compiler does not escape ``\`` in its output, so we use `sed` to replace it.

```bash
cd fe/compiler/tests/fixtures
fe -e=yul -o ./return_addition_u256 ./return_addition_u256.fe
cd ./return_addition_u256
sed -i 's/\\//g' out.yul
```

Then we could use SOLL to compile yul into ewasm bytecode

```bash
soll --runtime --target=EWASM -lang=Yul out.yul
```

### execute with rust-ssvm

To run with rust-ssvm, you could use fe-runner, an example in rust-ssvm.

```
cd rust-ssvm/example
cargo run -v --example rust-ssvm-fe-example <path to wasm> 0xae42e951 9 11
```
[\*] `0xae42e951` is the function signature of the targeting function, you are free to change it. To encode function signature, you could use API in web3.
```
web3.eth.abi.encodeFunctionSignature("bar(uint256,uint256)")
> 0xae42e951
```

You would find the output like this:

```
Output:  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20]
GasLeft: 49999982
Status:  EVMC_SUCCESS
Dump storage:
```

