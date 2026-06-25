#!/bin/bash

# Check that the repo is CI-clean.
# This must be kept in sync with .github/workflows/build.yml

set -e # exit on error
set -o xtrace

git submodule update --init --recursive

LOCKED="--locked"
if [ "$1" == "--unlocked" ]; then
    LOCKED=""
fi

kargo() {
    if [ "$1" == "+nightly" ]; then
        shift
        cargo +nightly $LOCKED "$@"
    else
        cargo $LOCKED "$@"
    fi
}

# Make sure the cross-compilation targets used by CI are installed.
rustup target add wasm32-unknown-unknown
rustup target add riscv32im-unknown-none-elf

g="-p groundhog-xdr"

# groundhog-xdr feature combinations
kargo build $g
kargo build $g --features=alloc
kargo build $g --features=std
kargo build $g --features=xdrs
kargo build $g --features=xdrs,syscalls_mock

CARGO_NET_GIT_FETCH_WITH_CLI=true \
    kargo +nightly doc -F syscalls_mock --no-deps $g

kargo build $g --features=serde
kargo build $g --features=serde,xdrs
kargo build $g --features=serde,alloc

# wasm32 / riscv32 cross-compilation
kargo build $g --target=wasm32-unknown-unknown
kargo build $g --features=xdrs,syscalls --target=wasm32-unknown-unknown
kargo build $g --features=xdrs,syscalls --target=riscv32im-unknown-none-elf
kargo build $g --target=wasm32-unknown-unknown --features=xdrs
kargo build $g --target=wasm32-unknown-unknown --features=alloc

kargo test --features=std

# Other workspace crates
kargo build -p groundhog_rpc
kargo build -p groundhog_scp
kargo build -p groundhog-trace
kargo build -p groundhog-spec

kargo fmt --check

kargo clippy -p groundhog-xdr-macros -- -D warnings

echo ">> CI clean!"
