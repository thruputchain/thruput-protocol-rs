# protocol-rs

Protocol bindings for Rust, generated via `x23c` from the XDR files in
`protocol/`.

## Regenerate rust sources

```shell
XDRC="../../hogd/build/x23/x23c/x23c" \
    bash scripts/generate.sh
```

## CI check:

```shell
bash scripts/ci-check.sh
```
