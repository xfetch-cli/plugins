# Changelog

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
