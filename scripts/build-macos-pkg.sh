#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-${VERSION:-}}"
TARGET="${2:-${TARGET:-}}"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "This script must be run on macOS" >&2
  exit 1
fi

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

if ! command -v pkgbuild >/dev/null 2>&1; then
  echo "pkgbuild not available" >&2
  exit 1
fi

PKG_ROOT=$(mktemp -d)
trap 'rm -rf "$PKG_ROOT"' EXIT

mkdir -p "$PKG_ROOT/usr/local/bin"
cp "$BIN_PATH" "$PKG_ROOT/usr/local/bin/freeagent"
chmod +x "$PKG_ROOT/usr/local/bin/freeagent"

mkdir -p dist
PKG_NAME="freeagent-${VERSION}-${TARGET}.pkg"

pkgbuild \
  --root "$PKG_ROOT" \
  --identifier "com.freeagent.cli" \
  --version "$VERSION" \
  --install-location "/" \
  "dist/$PKG_NAME"

if [ "${SKIP_CHECKSUMS:-}" != "1" ]; then
  ./scripts/checksums.sh
fi

echo "Created dist/$PKG_NAME"
