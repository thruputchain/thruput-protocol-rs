#!/bin/bash

if [ -z $XDRC ]; then
    echo "Must specify XDRC as command line variable (e.g. XDRC=\"path/to/xdrc\" ./generate.sh)"
    exit 1
fi

function generate() {
    for i in $@; do
        filename=$(basename $i .x)
        $XDRC --rust -o groundhog_xdr/src/$filename.rs $i
    done
}

generate protocol/types.x protocol/contract.x protocol/transaction.x protocol/ffi_enums.x
