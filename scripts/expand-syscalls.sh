#!/bin/bash

exec cargo expand \
    --package groundhog_xdr \
    --features syscalls \
    --color=always \
    syscalls
