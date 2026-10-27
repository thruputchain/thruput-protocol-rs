//! Compile-time pins of the generated RPC API signatures.
//!
//! Each module mirrors one workspace crate and contains a thin wrapper
//! function per generated RPC client/trait method, with the argument and
//! return types written out explicitly. The wrappers are never called;
//! compiling them is the assertion.
//!
//! If this crate fails to compile after a protocol bump, the RPC API
//! changed in a way `cargo-semver-checks` cannot see (e.g. a type swapped
//! inside a signature). Update the affected pin to the new signature AND
//! bump the workspace version to the next breaking level (see the
//! bump-crate-version skill). Do not "fix" a pin without considering the
//! version.
//!
//! This crate is `publish = false`. The publish workflow compiles it
//! explicitly (`cargo check -p thruput-api-pins`) because the publish
//! dry-run action skips private crates; regular CI compiles it via the
//! workspace-root `cargo test --features=std` in `scripts/ci-check.sh`.

pub mod coordinator;
pub mod db_gateway;
pub mod indexer;
pub mod node_rpc;
pub mod rpc;
pub mod scp;
pub mod view;
