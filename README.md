# protocol-rs

Rust bindings generated from the Groundhog protocol XDR definitions in
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
