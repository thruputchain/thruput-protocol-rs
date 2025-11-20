#!/bin/bash

exec cargo expand \
    --package groundhog_xdr \
    --features syscalls \
    "$@" \
    syscalls
