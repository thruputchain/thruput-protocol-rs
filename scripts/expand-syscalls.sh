#!/bin/bash

exec cargo expand \
    --package groundhog-xdr \
    --features syscalls \
    "$@" \
    syscalls
