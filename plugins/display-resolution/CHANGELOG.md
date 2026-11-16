# Changelog

## 2026-11-17 — 0.2.1

- Fixed xrandr parsing on Xwayland and multi-monitor setups: the geometry
  offset (`1920x1200+0+0`) is no longer mistaken for the resolution, and the
  bare `x` token from the rotation list is no longer reported as `x`.

## 2026-08-19
- Wrapped work in `with_timeout` with a 2 s budget; on timeout it responds with a fallback line.
