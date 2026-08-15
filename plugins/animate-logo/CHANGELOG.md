# Changelog

## 2026-08-15 — v0.1.0

### Full Frame Cycle for Frame-Style Animations

- Fixed frame truncation in `frame` style: when `duration_ms` is not set in the logo animation config, the plugin no longer caps the output to the 1200 ms default (e.g. 14 frames at 12 fps), which caused the animation to be cut short and restart from the beginning.
- When `duration_ms` is absent and `style` is `frame` with source frames available, the plugin now emits every source frame exactly once (e.g. all 36 frames of a kitty animation), letting the host loop the complete animation.
- Behavior for generated styles (`sweep`, `wave`, `rainbow`, `sparkle`, `breathing`, `none`) and for explicit `duration_ms` values is unchanged.
