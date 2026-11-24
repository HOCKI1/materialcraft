#!/bin/sh
set -eu

data_home="${XDG_DATA_HOME:-${HOME}/.local/share}"
cache_home="${XDG_CACHE_HOME:-${HOME}/.cache}"
: "${MATERIALCRAFT_DATA_DIR:=${data_home}/materialcraft-nightly}"
: "${MATERIALCRAFT_CACHE_DIR:=${cache_home}/materialcraft-nightly}"
export MATERIALCRAFT_DATA_DIR MATERIALCRAFT_CACHE_DIR

real_bin="$(dirname "$0")/.materialcraft-nightly/materialcraft"
exec "$real_bin" "$@"
