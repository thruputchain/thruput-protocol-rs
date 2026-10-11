# protocol-rs

Rust bindings generated from the Thruput protocol XDR definitions in
`protocol/` (a submodule of [groundhogchain/protocol]). The sources under each
crate's `src/` are emitted by hogd's `x23c` compiler and are not edited by
hand.

[groundhogchain/protocol]: https://github.com/groundhogchain/protocol

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

## Package names

New releases use `thruput_*` package names and the `thruput` Cargo registry
alias. Previously published `groundhog-*` and `groundhog_*` packages remain
available at their existing versions. To upgrade, change each dependency name
and Rust import explicitly, for example `groundhog-xdr` to `thruput_xdr` and
`groundhog_xdr::` to `thruput_xdr::`. Versions continue from the existing release
sequence; the first renamed release is 0.54.0.

`hogexec-internal` keeps its name. Repository URLs, protocol definitions, and
wire identifiers are unchanged.
