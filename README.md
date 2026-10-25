# protocol-rs

Rust bindings for the Thruput protocol: its data structures, contract syscall
interface and node RPC interfaces, generated from the protocol's XDR
definitions.

| Crate | Contents |
|-------|----------|
| `thruput-xdr` | Protocol types (transactions, contracts, events, storage, auth) and the contract syscall bindings |
| `thruput-xdr-macros` | Proc macros that generate the syscall bindings in `thruput-xdr` |
| `thruput-syscall` | Raw syscall dispatch ABI for the Thruput guest |
| `thruput-spec` | Contract spec types |
| `thruput-trace` | Transaction traces |
| `thruput-exec-internal` | FFI enums shared with the executor (invocation, host-function and system error codes), used by the trace bindings |
| `thruput-scp` | SCP RPC bindings |
| `thruput-coordinator`, `thruput-akkad` | Coordinator RPC bindings |
| `thruput-scheduler` | Scheduler RPC bindings |
| `thruput-indexer` | Indexer RPC bindings |
| `thruput-db-gateway` | Database read-gateway RPC bindings |
| `thruput-view` | Read-only view invocation RPC bindings |

Every crate shares one version and is published on crates.io. The bindings
encode with [`thruput-xdrs`] (imported as `xdrs`), and the RPC crates build
on [`thruput-rpc-api`]; serve or call them with [`thruput-rpc-driver`].

The minimum supported Rust version is 1.96. `thruput-xdr` is `no_std`; its
features:

- `alloc`, `std`: allocating types and `std` error impls (`std` implies
  `alloc`)
- `xdrs`: XDR encoding and decoding; decoding variable-length types also
  needs `alloc`
- `serde`: serde support, with fixed-size byte arrays as hex strings
- `schema`: JSON schemas through `schemars` (implies `std`)
- `syscalls`: contract syscall bindings, for `riscv32im-unknown-none-elf`
  guests
- `syscalls_mock`: the same bindings on a host target, against a native mock
  (implies `syscalls` and `std`)

[`thruput-xdrs`]: https://crates.io/crates/thruput-xdrs
[`thruput-rpc-api`]: https://crates.io/crates/thruput-rpc-api
[`thruput-rpc-driver`]: https://crates.io/crates/thruput-rpc-driver

## Generated code

The files `scripts/generate.sh` writes (most of each crate's `src/`) are
emitted by the `x23c` XDR compiler from the definitions in `protocol/`, a
submodule of [thruputchain/thruput-protocol], and are not edited by hand.
Each crate's `lib.rs`, its `impls` files and a few other modules, and the
`thruput-syscall`, `thruput-xdr-macros` and `thruput-api-pins` crates, are
hand-written.

The protocol definitions and `x23c` are not public yet, so only maintainers
can regenerate the sources. Everything else (building, testing, the CI
check) needs only the committed sources.

[thruputchain/thruput-protocol]: https://github.com/thruputchain/thruput-protocol

## CI check

```shell
bash scripts/ci-check.sh
```

This is what CI's build job runs. It needs no submodule or `x23c`.

## Regenerating the sources

With access to the protocol submodule, install the Python dependency in the
active environment, then regenerate:

```shell
git submodule update --init protocol
python3 -m pip install 'PyYAML==6.0.3'
bash scripts/generate.sh
```

Without `XDRC` set, `generate.sh` downloads the latest `x23c` release with
`gh`; set `XDRC` to use a local build instead. CI checks that the committed
sources are up to date with `bash scripts/generate.sh --check`.

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE)).
