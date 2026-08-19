# Changelog

## 2026-08-19
- Wrapped work in `with_timeout` with a 2 s budget; on timeout it responds with a fallback line.
- Documented platform support: Linux and macOS only (uses `mpc`/`playerctl`, unavailable on Windows).
