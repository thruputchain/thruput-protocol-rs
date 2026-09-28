# protocol-rs

Rust bindings generated from the Groundhog protocol XDR definitions in
`protocol/` (a submodule of [groundhogchain/protocol]). The sources under each
crate's `src/` are emitted by hogd's `x23c` compiler and are not edited by
hand.

Which crate you want:

- `groundhog-xdr` — the wire types (transactions, contracts, events, network
  config, and so on). Start here.
- `groundhog-spec` — contract spec types, `no_std`.
- `groundhog-syscall` and `groundhog-trace` — supporting crates for the
  contract syscall surface and execution traces.
- `groundhog_scheduler`, `groundhog_indexer`, `groundhog_db_gateway`,
  `groundhog_view`, `groundhog_coordinator`, `groundhog_scp`, `groundhog_hogx`,
  `groundhog_rpc`, `hogexec-internal`, and `groundhog-akkad` — bindings for
  hogd's internal x23 RPC services. They track hogd and are not a stable
  public API.

All crates publish to the Groundhog Cargo registry
(`sparse+https://crates.thruput.network/api/v1/crates/`; see
`.cargo/config.toml`). Every crate carries the workspace version, and the
`groundhog-xdr::PROTOCOL_VERSION` constant records the protocol submodule
revision the bindings were generated from.

[groundhogchain/protocol]: https://github.com/groundhogchain/protocol

## Regenerating (maintainers)

Regeneration needs `x23c` from a hogd build. When `XDRC` is unset,
`scripts/generate.sh` downloads it from the hogd `latest-x23c` GitHub release
with `gh`, so you need `gh` authenticated against `groundhogchain/hogd`. To use
a local build instead, build `x23c` in a sibling `hogd` checkout and point
`XDRC` at it. Either way, install the Python dependency in the active
environment:

```shell
python3 -m pip install 'PyYAML==6.0.3'
export XDRC="../../hogd/build/x23/x23c/x23c"   # optional; see above
```

Both regeneration and the local CI check use `XDRC`.

### Regenerate Rust sources

```shell
bash scripts/generate.sh
```

### CI check

```shell
bash scripts/ci-check.sh
```
