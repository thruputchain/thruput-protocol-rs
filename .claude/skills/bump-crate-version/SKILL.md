---
name: bump-crate-version
description: Use when bumping the workspace crate version (semver) in protocol-rs, not the protocol submodule commit
---

# Bump Crate Version

## 0. Choose the bump level (API pins)

Pre-1.0 semver: the middle number (0.25.0 → 0.26.0) is the breaking level; the
patch number is for compatible changes.

The `groundhog-api-pins` crate pins every generated RPC signature at compile
time. If it failed to compile — typically after a protocol submodule bump
swapped a type inside a signature — the RPC API changed in a way
`cargo-semver-checks` cannot detect (it has no lint for type changes inside
signatures). In that case:

- Update the affected pins in `groundhog-api-pins/src/` to the new signatures
  **in the same change**, and
- Bump the **middle** number here. Never update pins to appease CI without the
  corresponding breaking-level bump; the publish semver check will NOT catch
  the omission (the pin crate is `publish = false` and type swaps are its
  blind spot).

Rule of thumb: any diff under `groundhog-api-pins/` since the last release tag
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
groundhog-xdr = { path = "groundhog-xdr", version = "=X.Y.Z", ... }
# ... all path-based deps
```

**Internal deps** (bump these): `groundhog-xdr`, `groundhog-xdr-macros`, `groundhog-trace`, `groundhog_scp`, `groundhog_coordinator`, `groundhog_hogx`, `groundhog_rpc`, `groundhog_scheduler`, `hogexec-internal`, `macros`

**External deps** (DO NOT bump): `xdrs`, `rpc_driver`, `xdr-common`, `xdr-repr`, `xdr-derive`, `serde_with`, `serde_arrays`

## 3. Verify

```sh
cargo check
```

No other files need changing -- all member crates inherit `version.workspace = true`.
