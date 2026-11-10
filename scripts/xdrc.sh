# shellcheck shell=bash

# The xdrc that generates this repository's sources, pinned to a commit of
# thruputchain/xdrs. Sourced by generate.sh and compare-xdrc-api.sh.
#
# To move to a newer xdrc, change XDRC_REV, run scripts/generate.sh, and
# commit the regenerated sources with it.

XDRC_REPO=https://github.com/thruputchain/xdrs
XDRC_REV=900e8e94ddefe19817572df3a7fb730ec54fab89

# Prints the path to the pinned xdrc, building it on first use into a cache
# keyed by the commit.
xdrc_install() {
  local cache="${XDRC_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/thruput-protocol-rs/xdrc}"
  local root="${cache}/${XDRC_REV}"
  if [[ ! -x "${root}/bin/xdrc" ]]; then
    echo "-- Building xdrc ${XDRC_REV} from ${XDRC_REPO} --" >&2
    cargo install --quiet --locked --git "${XDRC_REPO}" --rev "${XDRC_REV}" \
      --root "${root}" --bin xdrc thruput-xdr-codegen >&2 || return 1
  fi
  printf '%s\n' "${root}/bin/xdrc"
}
