#!/bin/bash

[[ -z $XDRC ]] && XDRC=x23/x23c/x23c

printf "\tUsing XDRC=\"%s\"\n" $XDRC >&2

function generate() {
    for i in $@; do
        filename=$(basename $i .x)
        $XDRC --rust -o groundhog_xdr/src/$filename.rs $i
    done
}

generate protocol/types.x protocol/contract.x protocol/transaction.x protocol/ffi_enums.x
