#!/usr/bin/env bash
set -euo pipefail

if [ ! -d dist ]; then
  echo "dist directory not found" >&2
  exit 1
fi

checksum_cmd="sha256sum"
if ! command -v sha256sum >/dev/null 2>&1; then
  if command -v shasum >/dev/null 2>&1; then
    checksum_cmd="shasum -a 256"
  else
    echo "No sha256sum or shasum available" >&2
    exit 1
  fi
fi

(
  cd dist
  rm -f SHA256SUMS
  for file in *; do
    [ -f "$file" ] || continue
    case "$file" in
      SHA256SUMS) continue ;;
    esac
    $checksum_cmd "$file"
  done > SHA256SUMS
)

echo "Updated dist/SHA256SUMS"
