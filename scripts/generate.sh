#!/bin/bash

# XDR files in protocol/ from which to generate rust
protos=(
  types.x
  contract.x
  contract_error.x
  exec_result.x
  event.x
  ffi_enums.x
  genesis.x
  transaction.x
  retrocausal_queue.x
  storage_cost.x
  storage_delta_flags.x
  syscall_error.x
  auth_claim.x
  standard_asset_events.x
)

scps=(
  scp.x
  scp_config.x
  scp_control.x
  scp_handle.x
)

rpcs=(
  test_rpc.x
)

[[ -z $XDRC ]] && XDRC="../../hogd/build/x23/x23c/x23c"

echo "-- Using XDRC=${XDRC} --" >&2

generate() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust -o "${package}/src/${name}.rs" "${proto}"
}

for proto in "${protos[@]}"; do
  generate "protocol/${proto}" "groundhog_xdr"
done

for rpc in "${rpcs[@]}"; do
  generate "protocol/${rpc}" "groundhog_rpc"
done

for scp in "${scps[@]}"; do
  generate "protocol/${scp}" "groundhog_scp"
done
