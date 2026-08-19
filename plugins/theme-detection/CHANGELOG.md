# Changelog

## 2026-08-19
- Wrapped work in `with_timeout` with a 2 s budget; on timeout it responds with a fallback line.
- Added Windows support: light/dark mode and accent color read from the registry (HKCU Themes\Personalize and DWM).
