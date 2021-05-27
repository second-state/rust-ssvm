# Rust SSVM
![build](https://github.com/second-state/rust-ssvm/workflows/build/badge.svg)
[![Generic badge](https://img.shields.io/badge/Doc-master-green.svg)](https://second-state.github.io/rust-ssvm/rust_ssvm/index.html)

This project provides SSVM a Rust interface by using [EVMC](https://github.com/ethereum/evmc) (Ethereum Client-VM Connector API) to binding SSVM and host written in Rust.

We extend evmc rust binding module to include [evmc-client](https://github.com/second-state/evmc/releases/tag/v6.3.1-rust-evmc-client) base on evmc version 6.3.1 for now.

The software architecture of **evmc-client** was inspired by [go-ethereum](https://github.com/ethereum/go-ethereum) about how to use EVMC connect Ewasm VM ([Hera](https://github.com/ewasm/hera)) with host written in golang.


- EVMC-Client design architecture
```
evmc-client :                                ┌───────────────────────────────────────────────┐
stack diagram                                │                 evmc-client                   │
                                             ├───────────────────────────────────────────────┤
                                             │            lib.rs (pub interface)             │
                                             ├─────────────────────────────────┬─────────────┤
                                             │   host.rs (hook host context)   │  loader.rs  │
go-ethereum :                                ┆                                               ┆
sequential diagram                           ┆                                               ┆
                                                                         ┌───────────────────┐
┌───────────────────┐ ┌────────────────────┐ ┌─────────────────────────┐ │     C module      │
│geth/core/vm/evm.go│ │geth/core/vm/evmc.go│ │evmc/bindings/.../evmc.go│ │ex. loader and hera│
└─────────┬─────────┘ └─────────┬──────────┘ └────────────┬────────────┘ └─────────┬─────────┘
 NewEVM   │ NewEVMC             │                         │                        │
─────────>│────────────────────>│                         │                        │
          │           CreateVM ─┤                         │                        │
          │                     │ Load                    │                      ╔═══════════╗
          │                     │ ───────────────────────>│                      ║  Loader  ░║
          │                     │                         │ evmc_load_and_create ╚═══════════╝
          │                     │                         │───────────────────────>│
          │                     │                         │      load EVMC VM .so ─┤
          │                     │                         │      call evmc_create ─┤
          │                     │                         │                        │
          │                     │ return Instance{handle} │ return evmc_instance   │
          │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│
          │                     │                         │                        │
 run      │ Run                 │                         │                        │
─────────>│────────────────────>│                         │                      ╔═══════════╗
          │                     │ Execute                 │                      ║ EVMC VM  ░║
          │                     │ ───────────────────────>│                      ║ ex. Hera  ║
          │                     │                         │ evmc_execute         ╚═══════════╝
          │                     │                         │───────────────────────>│
          │                     │                         │               execute ─┤
          │                     │ return output, gasLeft, │                        │
          │                     │        err              │return evmc_result      │
          │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│
          │                     │                         │                        │
```

## Build environment

Build rust-ssvm you need prepare a build ssvm environment and include rust compile tool.

- Reuse ssvm build environment docker image
```bash
> docker pull secondstate/ssvm
```

- Fetch rust-ssvm source code
```bash
> git clone --recursive git@github.com:second-state/rust-ssvm.git
```

- Start docker container
```bash
> docker run -it \
    -v $(pwd)/rust-ssvm:/root/rust-ssvm \
    secondstate/ssvm:latest
```

- Install rust compile tool
```bash
(docker) $ curl https://sh.rustup.rs -sSf | sh
(docker) $ source ~/.cargo/env
```

- Build rust-ssvm lib
```bash
(docker) $ cd ~/rust-ssvm && cargo build -v
```

## Example
We provide a simple demo show how to launch a ssvm instance and call vm execute function with a dummy HostContext (host functions) and `fib.wasm` demo bytecode.

- The HostContext traits need implemented by chain's backend.  
- The `fib.wasm` was generated from [fib.yul](examples/fib.yul) written in Yul language.
> [Yul](https://solidity.readthedocs.io/en/latest/yul.html) is an intermediate language designed by Ethereum fundaction.  
> The other project of our company call [SOLL](https://github.com/second-state/SOLL) could help you compile Yul to EWASM. [(more...)](https://github.com/second-state/SOLL/blob/master/doc/guides/DevGuide.md#compile-and-execute-yul-code)

```bash
(docker) $ cd ~/rust-ssvm && cargo run --example execute_vm -v -- -f=examples/fib.wasm
```

The result should be the same as the following content.

```bash
Instantiate: ("ssvm", "0.4.0")
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000000" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000000" -> "0000000000000000000000000000000000000000000000000000000000000001"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000001" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000001" -> "0000000000000000000000000000000000000000000000000000000000000001"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000002" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000002" -> "0000000000000000000000000000000000000000000000000000000000000002"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000003" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000003" -> "0000000000000000000000000000000000000000000000000000000000000003"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000004" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000004" -> "0000000000000000000000000000000000000000000000000000000000000005"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000005" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000005" -> "0000000000000000000000000000000000000000000000000000000000000008"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000006" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000006" -> "000000000000000000000000000000000000000000000000000000000000000d"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000007" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000007" -> "0000000000000000000000000000000000000000000000000000000000000015"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000008" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000008" -> "0000000000000000000000000000000000000000000000000000000000000022"
Host: get_storage
"0000000000000000000000000000000000000000000000000000000000000009" -> "0000000000000000000000000000000000000000000000000000000000000000"
Host: set_storage
"0000000000000000000000000000000000000000000000000000000000000009" -> "0000000000000000000000000000000000000000000000000000000000000037"
Dump storage:
"0000000000000000000000000000000000000000000000000000000000000000" -> "0000000000000000000000000000000000000000000000000000000000000001"
"0000000000000000000000000000000000000000000000000000000000000001" -> "0000000000000000000000000000000000000000000000000000000000000001"
"0000000000000000000000000000000000000000000000000000000000000002" -> "0000000000000000000000000000000000000000000000000000000000000002"
"0000000000000000000000000000000000000000000000000000000000000003" -> "0000000000000000000000000000000000000000000000000000000000000003"
"0000000000000000000000000000000000000000000000000000000000000004" -> "0000000000000000000000000000000000000000000000000000000000000005"
"0000000000000000000000000000000000000000000000000000000000000005" -> "0000000000000000000000000000000000000000000000000000000000000008"
"0000000000000000000000000000000000000000000000000000000000000006" -> "000000000000000000000000000000000000000000000000000000000000000d"
"0000000000000000000000000000000000000000000000000000000000000007" -> "0000000000000000000000000000000000000000000000000000000000000015"
"0000000000000000000000000000000000000000000000000000000000000008" -> "0000000000000000000000000000000000000000000000000000000000000022"
"0000000000000000000000000000000000000000000000000000000000000009" -> "0000000000000000000000000000000000000000000000000000000000000037"
Output:  "0000000000000000000000000000000000000000000000000000000000000037"
GasLeft: 49800000
Status:  EVMC_SUCCESS
```

> If you want to see more runtime information inside SSVM, you can modify rust-ssvm/SSVM/lib/support/log.cpp as below.
```diff
 void setErrorLoggingLevel() {
   el::Loggers::addFlag(el::LoggingFlag::HierarchicalLogging);
-  el::Loggers::setLoggingLevel(el::Level::Error);
+  el::Loggers::setLoggingLevel(el::Level::Debug);
 }
```

## EWASM Test
Refer to the [EWASM Test Guide](./docs/EWASM_TEST.md) for more details.

## License
Rust SSVM has dual license, including [AGPL 3.0 license](LICENSE.AGPL-3.0) and [APACHE-2 license](LICENSE.APACHE-2).
