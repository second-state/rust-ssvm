# Rust SSVM

This project provides SSVM a Rust interface by using [EVMC](https://github.com/ethereum/evmc) (Ethereum Client-VM Connector API) to binding SSVM and host written in Rust.

The software architecture of rust-ssvm was inspired by [go-ethereum](https://github.com/ethereum/go-ethereum) about how to use EVMC connect Ewasm VM ([Hera](https://github.com/ewasm/hera)) with host written in golang.


- Design architecture
```
rust-ssvm :                                  ┌───────────────────────────────────────────────┐
stack diagram                                │                  rust-ssvm                    │
                                             ├───────────────────────────────────────────────┤
                                             │            lib.rs (pub interface)             │
                                             ├──────────────────────┬────────────┬───────────┤
                                             │  host.rs             │    SSVM    │ loader.rs │
go-ethereum :                                │  (hook host context) │            └───────────┤
sequential diagram
                                                                         ┌───────────────────┐
┌───────────────────┐ ┌────────────────────┐ ┌─────────────────────────┐ │     C module      │
│geth/core/vm/evm.go│ │geth/core/vm/evmc.go│ │evmc/bindings/.../evmc.go│ │ex. loader and hera│
└─────────┬─────────┘ └─────────┬──────────┘ └────────────┬────────────┘ └─────────┬─────────┘
 NewEVM   │ NewEVMC             │                         │                        │
─────────>│────────────────────>│                         │                        │
          │           CreateVM ─┤                         │                        │
          │                     │ Load                    │                      ╔═══════════╗
          │                     │ ───────────────────────>│                      ║Loader    ░║
          │                     │                         │ evmc_load_and_create ╚═══════════╝
          │                     │                         │────────────────────────>│
          │                     │                         │       load EVMC VM .so ─┤
          │                     │                         │       call evmc_create ─┤
          │                     │                         │                         │
          │                     │ return Instance{handle} │ return evmc_instance    │
          │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │
          │                     │                         │                         │
 run      │ Run                 │                         │                         │
─────────>│────────────────────>│                         │                      ╔═══════════╗
          │                     │ Execute                 │                      ║EVMC VM   ░║
          │                     │ ───────────────────────>│                      ║ex. Hera   ║
          │                     │                         │ evmc_execute         ╚═══════════╝
          │                     │                         │────────────────────────>│
          │                     │                         │                execute ─┤
          │                     │ return output, gasLeft, │                         │
          │                     │        err              │return evmc_result       │
          │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │
          │                     │                         │                         │
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
We provide a simple demo show how to launch a ssvm instance and call vm execute function with a dummy HostContext (host functions) and code with only wasm header pattern.
> The HostContext traits need implemented by chain's backend.

```shell
(docker) $ cd ~/rust-ssvm && cargo run --example execute_vm -v
```

> If you want to see ssvm log more detail, you can modify rust-ssvm/SSVM/lib/support/log.cpp as below.
```diff
 void setErrorLoggingLevel() {
   el::Loggers::addFlag(el::LoggingFlag::HierarchicalLogging);
-  el::Loggers::setLoggingLevel(el::Level::Error);
+  el::Loggers::setLoggingLevel(el::Level::Debug);
 }
```

## License
Rust SSVM is [AGPL 3.0 licensed](LICENSE).
