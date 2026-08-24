#!/bin/bash

# Thin wrapper kept for existing workflows and habits. The single
# source of truth for CI checks is scripts/ci-check.sh; this runs it
# without --locked.
exec "$(dirname "$0")/ci-check.sh" --unlocked
