#!/bin/bash

# Check that the repo is CI-clean.
# This is the single source of truth for the build job's checks:
# .github/workflows/build.yml runs this script, and
# scripts/workflow-check.sh is a thin wrapper around it.

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
rustup toolchain install "$NIGHTLY_TOOLCHAIN" --profile minimal --component rustfmt --component miri --component rust-src

g=(-p thruput-xdr)

# thruput-xdr feature combinations
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
kargo test "${g[@]}" --features=std,serde

# Other workspace crates
kargo build -p thruput-rpc
kargo build -p thruput-scp
kargo build -p thruput-trace
kargo build -p thruput-spec
kargo build -p thruput-view

cargo +"$NIGHTLY_TOOLCHAIN" fmt --check

# The generated client's call path under Miri: over a LoopbackLink the reply
# is decoded on the handler's task, into the client's future, while it waits.
MIRIFLAGS="-Zmiri-disable-isolation" \
    cargo +"$NIGHTLY_TOOLCHAIN" "${LOCKED[@]}" miri test -p thruput-rpc --test call_path

kargo clippy -p thruput-xdr-macros -- -D warnings

# Clippy over thruput-xdr, including tests. The two allowed lints
# (match_single_binding, large_enum_variant) fire only inside the
# x23c-generated sources, which must not be edited by hand.
kargo clippy -p thruput-xdr --all-targets --features=std,xdrs,serde -- \
    -D warnings \
    -A clippy::match-single-binding \
    -A clippy::large-enum-variant

echo ">> CI clean!"
