#!/bin/bash

exec cargo expand \
    --package thruput_xdr \
    --features syscalls \
    "$@" \
    syscalls
