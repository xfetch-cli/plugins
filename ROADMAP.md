# Plugin Ecosystem Roadmap

Status-driven roadmap for the official plugin registry. Each entry reflects
work already in the repository; the "open" items are direction, not commitments.

## Done

- [x] Core plugin system (`info_plugins` / `logo_animation`) — `xfetch plugin install/list/remove`
- [x] Official plugins for every platform: `animate-logo`, `chocolatey`, `display-resolution`, `docker`, `github-stats`, `music-player`, `temperature`, `theme-detection`, `theme-manager`, `timezone`, `user-info`, `weather`
- [x] Chocolatey moved out of the core into a plugin (v0.6.0)
- [x] Platform compatibility matrix (`docs/compatibility.md`)
- [x] Timeout standard: every official plugin wraps its work in `with_timeout` (enforced by CI)
- [x] Parallel plugin loading in the core (API untouched)
- [x] Community plugins: the SDK (`xfetch-plugin-api`) is stable; new plugins are
      welcome via the contribution process in `CONTRIBUTING.md`