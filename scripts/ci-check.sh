#!/bin/bash

# Check that the repo is CI-clean
# (if this is kept in sync with .github/workflows/build.yml)

set -e # exit on error
set -o xtrace

git submodule update --init --recursive

export RUSTFLAGS="--deny warnings"

LOCKED="--locked"
if ["$1" == "--unlocked"]; then
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

g="-p groundhog-xdr"
kargo build $g
kargo build $g -F alloc
kargo build $g -F std
kargo build $g -F spec
kargo build $g -F xdrs
kargo build $g -F xdrs,syscalls_mock
kargo test $g -F std

RUSTDOCFLAGS="--deny warnings --cfg docsrs" \
    kargo +nightly doc \
        -p groundhog-xdr \
        -F syscalls \
        --no-deps \

kargo build -p groundhog_rpc
kargo build -p groundhog_scp
kargo build -p groundhog_coordinator
kargo build -p groundhog_hogx

kargo fmt --check

kargo test -p groundhog_xdr_syscall_tests

echo ">> CI clean!"
