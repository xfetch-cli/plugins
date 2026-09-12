# Changelog

## 2026-09-12 — WebAssembly examples

### Wasm plugins

- Added four WebAssembly info providers, one per language, each with an `xfetch-plugin.json` manifest and capabilities declared explicitly:
  - `wasm-crypto` — Rust core module quoting BTC/ETH/... through the allowlisted HTTP host call.
  - `wasm-ip-geo` — Python component (componentize-py) showing public IP, location, network and timezone from ipapi.co.
  - `wasm-pacman` — Go core module counting repository vs AUR packages via allowlisted `pacman` exec, using the Go wasm host bridge (`//go:wasmimport`/`//go:wasmexport`).
  - `wasm-proc` — freestanding C core module (no wasi-libc) reading load average, uptime and memory from a read-only `/proc` preopen.
- Added `scripts/ci-wasm.sh`: local CI that builds every example with whichever toolchains are installed.
- Fixed the broken swap glyph in the animate-logo `layout_dots_full.jsonc` preset (U+FFFD replaced with `nf-md-swap_horizontal`).


## 2026-08-19

### Timeout Standard

- All plugins now wrap their work in `with_timeout` with an own runtime budget (2–25 s); on timeout they respond with fallback lines or exit gracefully, so a hung plugin can never hang xfetch.
- A plugin without a runtime limit is rejected — enforced by CI (`ci/unix.sh`, `ci/windows.ps1`, running on Linux, macOS and Windows). PRs must pass CI.
- Requires `xfetch-plugin-api` with `with_timeout` (see the `api` repo).

### Plugins (as of 2026-08-19)

- `animate-logo` — logo animation for the daemon
- `chocolatey` — count of packages installed via Chocolatey (Windows)
- `display-resolution` — screen resolution
- `docker` — container stats
- `github-stats` — GitHub profile statistics
- `music-player` — MPD/Spotify status
- `temperature` — CPU/thermal zone temperatures
- `theme-detection` — GTK theme detection
- `theme-manager` — theme registry management
- `timezone` — local time and UTC offset
- `user-info` — user, host and groups
- `weather` — weather via wttr.in

Each plugin has its own CHANGELOG with its specific budget and changes.
