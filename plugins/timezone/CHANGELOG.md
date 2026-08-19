# Changelog

## 2026-08-19
- Wrapped work in `with_timeout` with a 2 s budget; on timeout it responds with a fallback line.
- Added Windows support: local time and timezone id/offset via PowerShell (Get-Date, Get-TimeZone). The `format` arg is Linux/macOS only.
