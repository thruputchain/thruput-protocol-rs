---
name: bump-crate-version
description: Use when bumping the workspace crate version (semver) in protocol-rs, not the protocol submodule commit
---

# Bump Crate Version

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
