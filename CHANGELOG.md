# Changelog

## 2026-11-07 — banner-text 0.1.2

- Reverted the zero-width padding added in 0.1.1: it rendered as tofu squares
  on terminals or fonts without a U+200B glyph. The plugin emits the text
  unchanged again; the box border alignment remains a core layout issue.

## 2026-11-06 — SDK 0.3.1 migration

- Updated the workspace to `xfetch-plugin-api` 0.3.1 and
  `xfetch-guest-api` 0.3.1.
- SDK dependencies now resolve from crates.io; the temporary git branch is no
  longer required.
- The JSON protocol remains version 1 and the WIT remains
  `xfetch:runtime@0.1.0`.
- 0.3.1 removes an unused direct `serde` dependency from
  `xfetch-guest-api`; plugin code is unchanged.

## 2026-11-06 — Banner width fix

- `banner` 0.2.1: centre the proverb against the visible logo width using
  Unicode terminal width instead of counting ANSI escape bytes. Coloured logos
  no longer widen the logo block or push the box layout to the right.

## 2026-11-06 — CI and wasm validation

- Added a `Tests` workflow: fmt, clippy, workspace tests and catalog
  verification on pull requests and on main after merge. It does not build or
  publish release artifacts.
- Validated the wasm guests with their native toolchains: Rust
  (`wasm-crypto`), componentize-py (`wasm-ip-geo`), Go (`wasm-pacman`) and
  clang/wasm-ld (`wasm-proc`), including runtime checks against xfetch 1.0.3.

## 2026-11-06 — SDK 0.3.0 migration

- Updated the workspace to `xfetch-plugin-api` 0.3.0 and
  `xfetch-guest-api` 0.3.0.
- The JSON protocol remains version 1 and the WIT remains
  `xfetch:runtime@0.1.0`.
- Plugin code, manifests and catalog versions are unchanged; this is a
  dependency alignment with the core 0.3.0 SDK.

## 2026-11-03 — v1.0.2

Release train v1.0.2 (catalog schema 1, catalog_version 1.0.2).

### Dependency updates

- Updated the workspace lockfile with semver-compatible patches: `libc` 0.2.191, `libredox` 0.1.25, `objc2` 0.6.5, `redox_users` 0.5.3, `serde_json` 1.0.154, `syn` 3.0.9, `thiserror` 2.0.21, `thiserror-impl` 2.0.21 and `unicode-ident` 1.0.27.
- `gauges` 0.2.1: moved to `sysinfo` 0.40.1 and adapted the fallible constructors and the removal of `Disks::list()`. The measurements and output format are unchanged.
- `theme-manager` 0.2.1: moved to `dirs` 7.0.0. The `dirs::config_dir()` call is unchanged.
- Updated the security-audit workflow to `actions/checkout` 7.0.1 and the pinned `dtolnay/rust-toolchain` commit.
- Regenerated `index.json` so the catalog reports the new `gauges` and `theme-manager` versions.

### Addon catalog index

- Added the root `index.json`, a machine-readable addon catalog consumed by `xfetch plugin search` and the other remote addon discovery commands.
- Added `scripts/generate_catalog.py` to regenerate the catalog from each plugin's `Cargo.toml` or `xfetch-plugin.json`; `--check` fails when the committed index is stale.
- The catalog lists all 20 addons, sorted by name. Native entries use `runtime: "native"` and WebAssembly entries use `runtime: "wasm"` with their manifest capabilities preserved.
- Documented `banner`, `gauges` and `langs` in the README and platform compatibility tables. All three were already present in `index.json`.
- Crate versions remain 0.2.0 except `banner-text` 0.1.0, `gauges` 0.2.1 and `theme-manager` 0.2.1; the catalog version is 1.0.2.

## 2026-10-24 — Dependency security automation

- Added `.github/workflows/security-audit.yml`: runs `cargo audit` 0.22.2 with `--deny warnings` on push, pull request and a weekly schedule, with actions pinned by commit SHA and no persisted checkout credentials.
- Added weekly Dependabot updates for Cargo dependencies and GitHub Actions (`open-pull-requests-limit: 5`, minor/patch grouped).
- No plugin code, manifests or presets changed. The current lockfile audits clean (68 crates, 0 vulnerabilities, 0 warnings).

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
