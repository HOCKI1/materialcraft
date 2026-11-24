#!/usr/bin/env bash
# Validate the staged Arch launcher without building or opening MaterialCraft.
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
stage="$work/source"
dst="$work/dist"
base=materialcraft-0.0.1-nightly.1-linux-x86_64
mkdir -p "$stage/bin" "$dst" "$work/extract"

# A local argv recorder stands in for the compiled binaries. It never opens a UI.
cat >"$stage/bin/materialcraft" <<'RECORDER'
#!/bin/sh
printf '<%s>\n' "$@"
RECORDER
chmod 755 "$stage/bin/materialcraft"
cp "$stage/bin/materialcraft" "$stage/bin/materialcraft-cli"

bash "$HERE/nightly-arch-tar.sh" "$stage" "$work/stage" "$dst" "$base"
tar -xzf "$dst/$base.tar.gz" -C "$work/extract"
bin="$work/extract/$base/bin"
# Simulate the existing Arch bin-tree copy and public launcher rename.
mkdir -p "$work/installed"
cp -R "$bin" "$work/installed/bin"
bin="$work/installed/bin"
mv "$bin/materialcraft" "$bin/materialcraft-nightly"
mv "$bin/materialcraft-cli" "$bin/materialcraft-nightly-cli"

test ! -e "$stage/bin/.materialcraft-nightly"
cmp "$stage/bin/materialcraft" "$bin/.materialcraft-nightly/materialcraft"
cmp "$stage/bin/materialcraft-cli" "$bin/materialcraft-nightly-cli"
expect() {
  expected="$1"
  shift
  actual=$(PATH="$bin:$PATH" materialcraft-nightly "$@")
  if [[ "$actual" != "$expected" ]]; then
    printf 'expected:\n%s\nactual:\n%s\n' "$expected" "$actual" >&2
    exit 1
  fi
}
expect '<>'
expect "$(printf '<%s>\n' 'material with spaces.mcraft' '--other-name.mcraft')" \
  'material with spaces.mcraft' '--other-name.mcraft'
expect '<--version>' --version
expect '<-V>' -V

echo "PASS: Arch Nightly handoff preserves launch arguments"
