#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-${VERSION:-}}"
if [ -z "$VERSION" ]; then
  VERSION=$(awk -F ' = ' '/^version =/ { gsub(/"/, "", $2); print $2; exit }' Cargo.toml)
fi

if [ ! -f dist/SHA256SUMS ]; then
  echo "dist/SHA256SUMS not found. Run packaging first." >&2
  exit 1
fi

lookup_sha() {
  local suffix="$1"
  local match
  match=$(grep "${suffix}$" dist/SHA256SUMS | awk '{print $1}' | head -n1)
  if [ -z "$match" ]; then
    echo "Missing checksum for ${suffix}" >&2
    exit 1
  fi
  echo "$match"
}

SHA_MAC_ARM=$(lookup_sha "aarch64-apple-darwin.tar.gz")
SHA_MAC_X86=$(lookup_sha "x86_64-apple-darwin.tar.gz")
SHA_LINUX_ARM=$(lookup_sha "aarch64-unknown-linux-gnu.tar.gz")
SHA_LINUX_X86=$(lookup_sha "x86_64-unknown-linux-gnu.tar.gz")

sed \
  -e "s/__VERSION__/${VERSION}/g" \
  -e "s/__SHA256_MAC_ARM__/${SHA_MAC_ARM}/g" \
  -e "s/__SHA256_MAC_X86__/${SHA_MAC_X86}/g" \
  -e "s/__SHA256_LINUX_ARM__/${SHA_LINUX_ARM}/g" \
  -e "s/__SHA256_LINUX_X86__/${SHA_LINUX_X86}/g" \
  packaging/homebrew/freeagent.rb > dist/freeagent.rb

echo "Generated dist/freeagent.rb"
