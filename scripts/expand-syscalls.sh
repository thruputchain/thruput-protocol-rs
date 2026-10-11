#!/bin/bash

exec cargo expand \
    --package thruput-xdr \
    --features syscalls \
    "$@" \
    syscalls
