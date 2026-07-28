# protocol-rs

Protocol bindings for Rust, generated via `x23c` from the XDR files in
`protocol/`.

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
