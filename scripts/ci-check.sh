#!/bin/bash

# Check that the repo is CI-clean
# (if this is kept in sync with .github/workflows/build.yml)

set -e # exit on error
set -o xtrace

git submodule update --init --recursive

export RUSTFLAGS="--deny warnings"

g="-p groundhog_xdr"
cargo build $g
cargo build $g -F alloc
cargo build $g -F std
cargo build $g -F spec
cargo build $g -F syscalls
cargo test $g -F std

cargo build -p groundhog_rpc
cargo build -p groundhog_scp

cargo fmt --check

echo ">> CI clean!"
