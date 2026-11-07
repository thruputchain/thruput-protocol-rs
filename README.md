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
| `thruput-trace` | Transaction traces, and the FFI enums shared with the executor (invocation, host-function and system error codes) |
| `thruput-scp` | SCP RPC bindings |
| `thruput-node-types` | Types shared by the node RPC crates: transaction batches, block proposals, consensus values and common RPC results |
| `thruput-coordinator` | Coordinator RPC bindings |
| `thruput-node-rpc` | Node-internal RPC bindings: scheduler, database, executor, event, mempool/auth and gateway services |
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
emitted by the `xdrc` XDR compiler, from [thruputchain/xdrs], from the
definitions in `protocol/`, a submodule of [thruputchain/thruput-protocol].
They are not edited by hand.
Each crate's `lib.rs`, its `impls` files and a few other modules, and the
`thruput-syscall`, `thruput-xdr-macros` and `thruput-api-pins` crates, are
hand-written.

[thruputchain/thruput-protocol]: https://github.com/thruputchain/thruput-protocol
[thruputchain/xdrs]: https://github.com/thruputchain/xdrs
[thruputchain/hogd]: https://github.com/thruputchain/hogd

## CI check

```shell
bash scripts/ci-check.sh
```

This is what CI's build job runs. It needs neither the submodule nor `xdrc`.

## Regenerating the sources

Install the Python dependency in the
active environment, then regenerate:

```shell
git submodule update --init protocol
python3 -m pip install 'PyYAML==6.0.3'
bash scripts/generate.sh
```

`generate.sh` uses the `xdrc` pinned in `scripts/xdrc.sh`, which it builds
with `cargo install` on first use and caches under
`~/.cache/thruput-protocol-rs/xdrc`. Set `XDRC` to use another build, such
as a local one. To move to a newer `xdrc`, change the commit in
`scripts/xdrc.sh` and commit the regenerated sources with it. CI checks that
the committed sources are up to date with `bash scripts/generate.sh --check`.

### Comparing xdrc with x23c

`xdrc` replaced the `x23c` compiler from [thruputchain/hogd]. To check that
a version of `xdrc` generates the same public API as `x23c`, run:

```shell
bash scripts/compare-xdrc-api.sh
```

It generates the sources with each compiler, describes every crate's public
API from rustdoc's JSON output (with the nightly toolchain), and prints any
difference. By default it compares the pinned `xdrc` with the latest `x23c`
release; `--x23c` and `--xdrc` select other builds. `--docs` and `--attrs`
also compare doc comments and attributes; one known difference in the docs
is expected, which the script's header describes. CI does not run it.

## License

Copyright 2026 Stellar Development Foundation. Licensed under the Apache
License, Version 2.0. Each published crate includes the workspace
[LICENSE](LICENSE). The `thruput-scp` crate also includes a `NOTICE` file
with its upstream attribution.
