#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-${VERSION:-}}"
TARGET="${2:-${TARGET:-}}"

if [ -z "$VERSION" ]; then
  VERSION=$(awk -F ' = ' '/^version =/ { gsub(/"/, "", $2); print $2; exit }' Cargo.toml)
fi

if [ -z "$TARGET" ]; then
  TARGET=$(rustc -vV | awk -F ': ' '/host/ { print $2; exit }')
fi

BIN_PATH="target/${TARGET}/release/freeagent"
if [ ! -f "$BIN_PATH" ]; then
  BIN_PATH="target/release/freeagent"
fi

if [ ! -f "$BIN_PATH" ]; then
  echo "Binary not found at target/${TARGET}/release/freeagent or target/release/freeagent" >&2
  exit 1
fi

mkdir -p dist

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$BIN_PATH" "$TMP_DIR/freeagent"
chmod +x "$TMP_DIR/freeagent"
cp README.md "$TMP_DIR/"

TARBALL="freeagent-${VERSION}-${TARGET}.tar.gz"

tar -C "$TMP_DIR" -czf "dist/$TARBALL" freeagent README.md

if [ "${SKIP_CHECKSUMS:-}" != "1" ]; then
  ./scripts/checksums.sh
fi

echo "Created dist/$TARBALL"
