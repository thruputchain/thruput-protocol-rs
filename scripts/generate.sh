#!/bin/bash

# XDR files in protocol/ from which to generate rust for groundhog-xdr
protos=(
  types.x
  contract.x
  exec_result.x
  event.x
  genesis.x
  transaction.x
  invoke_flag.x
  invoke_args.x
  retrocausal_queue.x
  storage_cost.x
  network_config.x
  syscall_error.x
  auth_claim.x
  standard_asset_events.x
  address_auth.x
  storage_item.x
  precompiles.x
  auth_token.x
  view.x
  preimage.x
  header.x
  validator.x
  network.x
)

traces=(
  db_conflict.x
  transaction_trace.x
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
)

rpcs=(
  test_rpc.x
)

coordinators=(
  discovery.x
  config.x
  dogwood_kit.x
  dogwood.x
)

# dependencies for the rust coordinator
akkads=(
  batch.x
  bristlecone.x
  catchup.x
  config.x
  consensus.x
  coord_sched.x
  db_sched.x
  discovery.x
  dogwood_kit.x
  dogwood.x
  integration.x
  loadgen.x
  multiplex.x
  proposal.x
  parallel.x
  repair2.x
  rpc_types.x
  sched_coord.x
)

hogxs=(
    port.x
    hogx_agent.x
    hogx_base.x
    hogx_cluster.x
    hogx_host.x
    hogx_unit.x
    hogx_validator.x
)

specs=(
  groundhog_contract_spec.x
)


# Without XDRC, download the latest x23c release the way CI does.
if [[ -z $XDRC ]]; then
  os=$(uname -s | tr '[:upper:]' '[:lower:]')
  [[ $os == darwin ]] && os=macos
  arch=$(uname -m)
  asset="x23c-${os}-${arch}.tar.gz"
  xdrc_tmpdir=$(mktemp -d)
  trap 'rm -rf "$xdrc_tmpdir"' EXIT
  echo "-- Downloading ${asset} from groundhogchain/hogd latest-x23c release --" >&2
  if ! gh release download latest-x23c \
         --repo groundhogchain/hogd \
         --pattern "${asset}" \
         --dir "$xdrc_tmpdir"; then
    echo "error: failed to download ${asset} from the latest-x23c release; set XDRC to a local x23c build" >&2
    exit 1
  fi
  tar xzf "$xdrc_tmpdir/$asset" -C "$xdrc_tmpdir"
  chmod +x "$xdrc_tmpdir/x23c"
  XDRC="$xdrc_tmpdir/x23c"
fi

echo "-- Using XDRC=${XDRC} --" >&2

if [[ ! -x "${XDRC}" ]]; then
  echo "error: x23c is not executable at ${XDRC}; build hogd's x23c or set XDRC" >&2
  exit 1
fi
if ! python3 -c 'import yaml' >/dev/null 2>&1; then
  echo "error: PyYAML is required; install PyYAML==6.0.3 in the active Python environment" >&2
  exit 1
fi

set -ex

generate() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust --rust-no-feature spec -o "${package}/src/${name}.rs" "${proto}"
}

generate_no_schema() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust --rust-no-feature schema --rust-no-feature spec -o "${package}/src/${name}.rs" "${proto}"
}

generate_rpc() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust --rust-feature xdrs --rust-feature schema --rust-no-feature spec --rust-no-feature serde-arrays -o "${package}/src/${name}.rs" "${proto}"
}

generate_trace() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust --rust-feature xdrs --rust-feature schema --rust-no-feature spec -o "${package}/src/${name}.rs" "${proto}"
}

# groundhog-spec is always no_std and has no `std` feature, so leave out what
# x23c would otherwise gate on one.
generate_contract_spec() {
  proto=${1:?}
  package=${2:?}
  name=$(basename "${proto}" .x)
  $XDRC --rust --rust-feature xdrs --rust-no-feature spec --rust-no-feature std -o "${package}/src/${name}.rs" "${proto}"
}

for proto in "${protos[@]}"; do
  generate "protocol/${proto}" "groundhog-xdr"
done

for trace in "${traces[@]}"; do
  generate_trace "protocol/${trace}" "groundhog-trace"
done

# x23c preserves this XDR fixed-opaque notation in rustdoc, where it is parsed
# as an intra-doc link and fails documentation builds with warnings denied.
perl -pi -e 's/ContractAddress\[32\]/`ContractAddress[32]`/' groundhog-trace/src/db_conflict.rs

for ffi_enum in "${hogexec_internal[@]}"; do
  generate_no_schema "protocol/${ffi_enum}" "hogexec-internal"
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

for akkad in "${akkads[@]}"; do
  generate_rpc "protocol/${akkad}" "groundhog-akkad"
done

for hogx in "${hogxs[@]}"; do
  generate_rpc "protocol/${hogx}" "groundhog_hogx"
done

for spec in "${specs[@]}"; do
  generate_contract_spec "protocol/${spec}" "groundhog-spec"
done

indexers=(
  indexer.x
  pagination.x
)

for idx in "${indexers[@]}"; do
  generate_rpc "protocol/${idx}" "groundhog_indexer"
done

scheduler_deps=(
  rpc_types.x
  parallel.x
  proposal.x
  db_sched.x
  sched_event.x
  event_rpc.x
  exec_sched.x
  consensus.x
)

schedulers=(
  sched_coord.x
  sched_gateway.x
  coord_sched.x
  catchup.x
  repair.x
  loadgen.x
)

for dep in "${scheduler_deps[@]}"; do
  generate_rpc "protocol/${dep}" "groundhog_scheduler"
done

for sched in "${schedulers[@]}"; do
  generate_rpc "protocol/${sched}" "groundhog_scheduler"
done

db_gateways=(
  db_gateway.x
)

for dg in "${db_gateways[@]}"; do
  generate_rpc "protocol/${dg}" "groundhog_db_gateway"
done

views=(
  view_gateway.x
)

for view in "${views[@]}"; do
  generate_rpc "protocol/${view}" "groundhog_view"
done

# Generate protocol version constant from the checked-out protocol submodule revision.
# ProtocolCommit is a fixed opaque[20], so the token must be exactly 20 bytes:
# the "hogp/" prefix leaves 15 for the commit id. Cut to width rather than
# using `rev-parse --short=15`, which treats 15 as a minimum and returns a
# longer prefix when one is needed for uniqueness.
PROTOCOL_HASH=$(git -C protocol rev-parse HEAD | cut -c1-15)
PROTOCOL_TOKEN="hogp/${PROTOCOL_HASH}"
if [[ ${#PROTOCOL_TOKEN} -ne 20 ]]; then
  echo "protocol token is ${#PROTOCOL_TOKEN} bytes, want 20: ${PROTOCOL_TOKEN}" >&2
  exit 1
fi
echo "-- Generating protocol version: ${PROTOCOL_TOKEN} --" >&2
cat > groundhog-xdr/src/protocol_version.rs <<PEOF
// Generated by generate.sh — do not edit
use crate::network::ProtocolCommit;

/// The token identifying the protocol submodule revision, in the fixed-width
/// form the wire carries.
///
/// Typed as [ProtocolCommit] rather than a string: the bytes are printable
/// ASCII, but their width is part of the protocol, and the byte-string literal
/// below fails to compile if it no longer fills the array.
pub const PROTOCOL_VERSION: ProtocolCommit = *b"${PROTOCOL_TOKEN}";
PEOF

# Generate syscall documentation
echo "-- Generating syscall documentation --" >&2
python3 protocol/generate_syscalls_doc.py --json-output groundhog-xdr/syscalls_docs.json
echo "-- Copying syscall.yaml to groundhog-xdr/src/syscalls.yaml --" >&2
cp protocol/syscalls.yaml groundhog-xdr/src/syscalls.yaml

if [[ "${1:-}" == "--check" ]]; then
  set +x
  generation_status=$(git status --porcelain --untracked-files=all)
  if [[ -n $generation_status ]]; then
    echo "Generated files are out of date:" >&2
    printf '%s\n' "$generation_status" >&2
    echo "Tracked diff summary:" >&2
    git diff --stat -- >&2
    exit 1
  fi
fi
