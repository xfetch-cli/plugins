#!/usr/bin/env bash
# Local CI for the WebAssembly plugin examples.
#
# Toolchains that are missing are skipped with a notice; the script fails only
# when an available toolchain cannot build its example. Run it from anywhere.
set -euo pipefail
cd "$(dirname "$0")/.."

rust_build() {
  if rustup target list --installed 2>/dev/null | grep -q '^wasm32-wasip1$'; then
    echo "==> cargo build --release --target wasm32-wasip1 ($1)"
    cargo build --release --target wasm32-wasip1 -p "$1"
  else
    echo "==> skip Rust wasm ($1): wasm32-wasip1 target not installed"
  fi
}

rust_build xfetch-plugin-wasm-crypto

if command -v componentize-py >/dev/null 2>&1; then
  echo "==> componentize-py (wasm-ip-geo)"
  (
    cd plugins/wasm-ip-geo
    mkdir -p dist
    componentize-py --quiet -d ../../../api/wit -w plugin componentize app -p . -o dist/wasm-ip-geo.wasm
  )
else
  echo "==> skip Python component: componentize-py not in PATH"
fi

if command -v go >/dev/null 2>&1; then
  echo "==> go build (wasm-pacman)"
  (
    cd plugins/wasm-pacman
    mkdir -p dist
    GOOS=wasip1 GOARCH=wasm go build -o dist/wasm-pacman.wasm .
  )
else
  echo "==> skip Go example: go not in PATH"
fi

if command -v clang >/dev/null 2>&1; then
  echo "==> clang (wasm-proc)"
  (cd plugins/wasm-proc && sh build.sh)
else
  echo "==> skip C example: clang not in PATH"
fi

echo "==> wasm plugins CI OK"
