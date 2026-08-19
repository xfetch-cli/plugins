#!/usr/bin/env bash
# CI for Linux/macOS: build, test and enforce the plugin standard.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo test --workspace

# Standard: every plugin must wrap its work in with_timeout (CONTRIBUTING.md).
for f in plugins/*/src/main.rs; do
  grep -q "with_timeout" "$f" || {
    echo "::error::$f must use xfetch_plugin_api::with_timeout" >&2
    exit 1
  }
done
echo "All plugins use with_timeout."
