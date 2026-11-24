#!/bin/sh
set -eu

real_bin="$(dirname "$0")/.materialcraft-nightly/materialcraft"
exec "$real_bin" "$@"
