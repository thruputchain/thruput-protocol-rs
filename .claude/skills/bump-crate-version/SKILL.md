---
name: bump-crate-version
description: Use when bumping the workspace crate version (semver) in protocol-rs, not the protocol submodule commit
---

# Bump Crate Version

## 0. Choose the bump level (API pins)

Pre-1.0 semver: the middle number (0.25.0 → 0.26.0) is the breaking level; the
patch number is for compatible changes.

The `thruput-api-pins` crate pins every generated RPC signature at compile
time. If it failed to compile — typically after a protocol submodule bump
swapped a type inside a signature — the RPC API changed in a way
`cargo-semver-checks` cannot detect (it has no lint for type changes inside
signatures). In that case:

- Update the affected pins in `thruput-api-pins/src/` to the new signatures
  **in the same change**, and
- Bump the **middle** number here. Never update pins to appease CI without the
  corresponding breaking-level bump; the publish semver check will NOT catch
  the omission (the pin crate is `publish = false` and type swaps are its
  blind spot).

Rule of thumb: any diff under `thruput-api-pins/` since the last release tag
means the middle number must be higher than the last published version.

Bump the semver version in the root `Cargo.toml`. Two places must be updated:

## 1. Workspace package version

```toml
[workspace.package]
version = "X.Y.Z"   # ← bump this
```

## 2. Internal workspace dependency versions

Every dependency under `[workspace.dependencies]` that uses `path = "..."` is internal and must match:

```toml
[workspace.dependencies]
thruput-xdr = { path = "thruput-xdr", version = "=X.Y.Z", ... }
# ... all path-based deps
```

**Internal deps** (bump these): `thruput-syscall`, `thruput-xdr`, `thruput-xdr-macros`, `thruput-trace`, `thruput-scp`, `thruput-coordinator`, `thruput-rpc`, `thruput-node-types`, `thruput-node-rpc`, `thruput-indexer`, `thruput-db-gateway`, `thruput-view`, `thruput-spec`

**External deps** (DO NOT bump): `xdrs`, `xdr-common`, `xdr-derive`, `thruput-rpc-api`, `thruput-rpc-driver`, `schemars`

If these lists disagree with `Cargo.toml`, the rule wins: bump every entry that has `path = "..."`, and nothing else. A missed internal pin fails `cargo check` with an unresolvable `=X.Y.Z` requirement.

## 3. Verify

```sh
cargo check
```

`cargo check` rewrites the workspace crates' versions in `Cargo.lock`; commit that with `Cargo.toml`, since CI builds with `--locked`. No other files need changing -- all member crates inherit `version.workspace = true`.
