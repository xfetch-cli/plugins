#!/usr/bin/env sh
# Builds the freestanding C wasm plugin. Requires clang with the WebAssembly
# backend and LLVM's wasm-ld (both shipped with LLVM).
set -eu

cd "$(dirname "$0")"
mkdir -p dist

clang \
  --target=wasm32-wasip1 \
  -O2 \
  -nostdlib \
  -fno-stack-protector \
  -Wl,--no-entry \
  -Wl,--export-memory \
  -Wl,--allow-undefined \
  -o dist/wasm-proc.wasm \
  main.c

echo "Built dist/wasm-proc.wasm"
