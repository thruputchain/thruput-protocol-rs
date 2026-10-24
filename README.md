# protocol-rs

Rust bindings generated from the Thruput protocol XDR definitions in
`protocol/` (a submodule of [thruputchain/thruput-protocol]). The files
`scripts/generate.sh` writes (most of each crate's `src/`) are emitted by
hogd's `x23c` compiler and are not edited by hand. Each crate's `lib.rs`, its
`impls` files and a few other modules, and the `thruput-syscall`,
`thruput-xdr-macros` and `thruput-api-pins` crates, are hand-written.

[thruputchain/thruput-protocol]: https://github.com/thruputchain/thruput-protocol

## Generation prerequisites

Build `x23c` in a sibling `hogd` checkout and install the Python dependency in
the active environment:

```shell
python3 -m pip install 'PyYAML==6.0.3'
export XDRC="../../hogd/build/x23/x23c/x23c"
```

Both regeneration and the local CI check use `XDRC`.

## Regenerate Rust sources

```shell
bash scripts/generate.sh
```

## CI check

```shell
bash scripts/ci-check.sh
```
