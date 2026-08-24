#!/bin/bash

# Check that the repo is CI-clean.
# This must be kept in sync with .github/workflows/build.yml

set -e # exit on error
set -o xtrace

git submodule update --init --recursive
./scripts/generate.sh --check

LOCKED=(--locked)
if [[ "${1:-}" == "--unlocked" ]]; then
    LOCKED=()
fi

readonly NIGHTLY_TOOLCHAIN="nightly-2026-05-26"

kargo() {
    cargo "${LOCKED[@]}" "$@"
}

# Make sure the cross-compilation targets used by CI are installed.
rustup target add riscv32im-unknown-none-elf
rustup toolchain install "$NIGHTLY_TOOLCHAIN" --profile minimal --component rustfmt

g=(-p groundhog-xdr)

# groundhog-xdr feature combinations
kargo build "${g[@]}"
kargo build "${g[@]}" --features=alloc
kargo build "${g[@]}" --features=std
kargo build "${g[@]}" --features=xdrs
kargo build "${g[@]}" --features=xdrs,syscalls_mock

CARGO_NET_GIT_FETCH_WITH_CLI=true \
    cargo +"$NIGHTLY_TOOLCHAIN" "${LOCKED[@]}" doc -F syscalls_mock --no-deps "${g[@]}"

kargo build "${g[@]}" --features=serde
kargo build "${g[@]}" --features=serde,xdrs
kargo build "${g[@]}" --features=serde,alloc

# riscv32 cross-compilation
kargo build "${g[@]}" --target=riscv32im-unknown-none-elf
kargo build "${g[@]}" --features=xdrs,syscalls --target=riscv32im-unknown-none-elf
kargo build "${g[@]}" --target=riscv32im-unknown-none-elf --features=xdrs
kargo build "${g[@]}" --target=riscv32im-unknown-none-elf --features=alloc

kargo test --features=std
kargo test "${g[@]}" --features=std,xdrs

# Other workspace crates
kargo build -p groundhog_rpc
kargo build -p groundhog_scp
kargo build -p groundhog-trace
kargo build -p groundhog-spec

cargo +"$NIGHTLY_TOOLCHAIN" fmt --check

kargo clippy -p groundhog-xdr-macros -- -D warnings

echo ">> CI clean!"
