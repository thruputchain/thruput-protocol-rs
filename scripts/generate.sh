#!/bin/bash

# XDR files in protocol/ from which to generate rust for groundhog-xdr
protos=(
  types.x
  contract.x
  exec_result.x
  event.x
  genesis.x
  transaction.x
  retrocausal_queue.x
  storage_cost.x
  syscall_error.x
  auth_claim.x
  standard_asset_events.x
  address_auth.x
  storage_item.x
  precompiles.x
)

# XDR files for hogexec-internal
hogexec_internal=(
  ffi_enums.x
)

scps=(
  scp.x
  scp_config.x
  scp_control.x
  scp_handle.x
  network.x
)

rpcs=(
  test_rpc.x
)

coordinators=(
  discovery.x
  config.x
  wallet_kit.x
  dogwood_kit.x
)

hogxs=(
    hogx_agent.x
    hogx_base.x
    hogx_cluster.x
    hogx_host.x
    hogx_unit.x
    hogx_validator.x
)

[[ -z $XDRC ]] && XDRC="../../hogd/build/x23/x23c/x23c"

echo "-- Using XDRC=${XDRC} --" >&2

set -ex

generate() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust -o "${package}/src/${name}.rs" "${proto}"
}

generate_rpc() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust --rust-feature xdrs --rust-no-feature spec --rust-no-feature serde-arrays -o "${package}/src/${name}.rs" "${proto}"
}

for proto in "${protos[@]}"; do
  generate "protocol/${proto}" "groundhog-xdr"
done

for ffi_enum in "${hogexec_internal[@]}"; do
  generate "protocol/${ffi_enum}" "hogexec-internal"
done

for rpc in "${rpcs[@]}"; do
  generate_rpc "protocol/${rpc}" "groundhog_rpc"
done

for scp in "${scps[@]}"; do
  generate_rpc "protocol/${scp}" "groundhog_scp"
done

for coordinator in "${coordinators[@]}"; do
  generate_rpc "protocol/${coordinator}" "groundhog_coordinator"
done

for hogx in "${hogxs[@]}"; do
  generate_rpc "protocol/${hogx}" "groundhog_hogx"
done

# Generate syscall documentation
echo "-- Generating syscall documentation --" >&2
python3 protocol/generate_syscalls_doc.py --json-output groundhog-xdr/syscalls_docs.json
echo "-- Copying syscall.yaml to groundhog-xdr/src/syscalls.yaml --" >&2
cp protocol/syscalls.yaml groundhog-xdr/src/syscalls.yaml
