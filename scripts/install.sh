#!/usr/bin/env bash
set -euo pipefail

REPO="${FREEAGENT_GITHUB_REPO:-amogower/freeagent-cli}"
VERSION="${1:-latest}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin) PLATFORM="apple-darwin" ;;
  Linux) PLATFORM="unknown-linux-gnu" ;;
  *) echo "Unsupported OS: $OS" >&2; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported arch: $ARCH" >&2; exit 1 ;;
esac

TARGET="${ARCH}-${PLATFORM}"

if [ "$VERSION" = "latest" ]; then
  if ! command -v python3 >/dev/null 2>&1; then
    echo "python3 is required to fetch the latest release" >&2
    exit 1
  fi
  RELEASE_JSON=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest")
  ASSET_URL=$(python3 - <<PY
import json, sys
release = json.loads(sys.stdin.read())
assets = release.get("assets", [])
needle = f"freeagent-"
needle_target = f"-{TARGET}.tar.gz"
for asset in assets:
    name = asset.get("name", "")
    if name.startswith(needle) and name.endswith(needle_target):
        print(asset.get("browser_download_url", ""))
        break
PY
  <<<"$RELEASE_JSON")
  SHA_URL=$(python3 - <<PY
import json, sys
release = json.loads(sys.stdin.read())
assets = release.get("assets", [])
for asset in assets:
    if asset.get("name") == "SHA256SUMS":
        print(asset.get("browser_download_url", ""))
        break
PY
  <<<"$RELEASE_JSON")
else
  ASSET_URL="https://github.com/${REPO}/releases/download/v${VERSION}/freeagent-${VERSION}-${TARGET}.tar.gz"
  SHA_URL="https://github.com/${REPO}/releases/download/v${VERSION}/SHA256SUMS"
fi

if [ -z "$ASSET_URL" ]; then
  echo "Could not find release asset for ${TARGET}" >&2
  exit 1
fi

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

TARBALL="$TMP_DIR/freeagent.tar.gz"
SHA_FILE="$TMP_DIR/SHA256SUMS"

curl -fL "$ASSET_URL" -o "$TARBALL"

if curl -fL "$SHA_URL" -o "$SHA_FILE"; then
  CHECKSUM_CMD="sha256sum"
  if ! command -v sha256sum >/dev/null 2>&1; then
    if command -v shasum >/dev/null 2>&1; then
      CHECKSUM_CMD="shasum -a 256"
    else
      echo "No sha256sum or shasum available; skipping checksum verification" >&2
      CHECKSUM_CMD=""
    fi
  fi

  if [ -n "$CHECKSUM_CMD" ]; then
    EXPECTED=$(grep "$(basename "$ASSET_URL")" "$SHA_FILE" | awk '{print $1}')
    if [ -n "$EXPECTED" ]; then
      ACTUAL=$($CHECKSUM_CMD "$TARBALL" | awk '{print $1}')
      if [ "$EXPECTED" != "$ACTUAL" ]; then
        echo "Checksum verification failed" >&2
        exit 1
      fi
    else
      echo "Checksum not found for asset; skipping verification" >&2
    fi
  fi
fi

INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
if [ ! -w "$INSTALL_DIR" ]; then
  INSTALL_DIR="$HOME/.local/bin"
fi

mkdir -p "$INSTALL_DIR"

tar -xzf "$TARBALL" -C "$TMP_DIR"
install -m 755 "$TMP_DIR/freeagent" "$INSTALL_DIR/freeagent"

echo "Installed freeagent to $INSTALL_DIR/freeagent"
