#!/bin/bash

set -x

fail() {
    echo "Failed!"
    exit 1
}

trap fail ERR;

readonly NIGHTLY_TOOLCHAIN="nightly-2026-05-26"

./scripts/generate.sh --check
rustup toolchain install "$NIGHTLY_TOOLCHAIN" --profile minimal --component rustfmt

cargo build -p groundhog-xdr
cargo build -p groundhog-xdr --features=alloc
cargo build -p groundhog-xdr --features=std
cargo build -p groundhog-xdr --features=xdrs
cargo build -p groundhog-xdr --features=xdrs,syscalls_mock
CARGO_NET_GIT_FETCH_WITH_CLI=true \
    cargo +"$NIGHTLY_TOOLCHAIN" doc -F syscalls_mock --no-deps -p groundhog-xdr
cargo build -p groundhog-xdr --features=serde
cargo build -p groundhog-xdr --features=serde,xdrs
cargo build -p groundhog-xdr --features=serde,alloc
cargo build -p groundhog-xdr --target=wasm32-unknown-unknown
cargo build -p groundhog-xdr --features=xdrs,syscalls --target=wasm32-unknown-unknown
cargo build -p groundhog-xdr --target=wasm32-unknown-unknown --features=xdrs
cargo build -p groundhog-xdr --target=wasm32-unknown-unknown --features=alloc
cargo test --features=std
cargo build -p groundhog_rpc
cargo build -p groundhog_scp
cargo build -p groundhog_view
cargo +"$NIGHTLY_TOOLCHAIN" fmt --check
cargo clippy -p groundhog-xdr-macros -- -D warnings
