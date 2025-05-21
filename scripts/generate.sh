#!/bin/bash

# XDR files in protocol/ from which to generate rust
protos=(
  types.x
  contract.x
  transaction.x
  ffi_enums.x
  retrocausal_queue.x
)

[[ -z $XDRC ]] && XDRC="../../hogd/build/x23/x23c/x23c"

echo "-- Using XDRC=${XDRC} --" >&2

generate() {
  proto=${1:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust -o "groundhog_xdr/src/${name}.rs" "${proto}"
}

for proto in "${protos[@]}"; do
  generate "protocol/${proto}"
done
