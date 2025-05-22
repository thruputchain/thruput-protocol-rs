#!/bin/bash

# Check that the repo is CI-clean
# (if this is kept in sync with .github/workflows/build.yml)

set -e # exit on error
set -o xtrace

git submodule update --init --recursive

g="--manifest-path groundhog_xdr/Cargo.toml"

cargo build $g
cargo build $g -F alloc
cargo build $g -F std
cargo test  $g -F std
cargo +nightly fmt $g --check
