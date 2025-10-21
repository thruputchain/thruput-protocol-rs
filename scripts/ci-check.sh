#!/bin/bash

# Check that the repo is CI-clean
# (if this is kept in sync with .github/workflows/build.yml)

set -e # exit on error
set -o xtrace

git submodule update --init --recursive

g="-p groundhog_xdr"
cargo build $g
cargo build $g -F alloc
cargo build $g -F std
cargo build $g -F spec
cargo test $g -F std
cargo fmt $g --check

cargo build -p groundhog_rpc
cargo build -p groundhog_scp
