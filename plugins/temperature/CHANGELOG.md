# Changelog

## 2026-08-18 — v0.1.0

### Initial Release

- New `temperature` info plugin: reads kernel thermal zones (`/sys/class/thermal/thermal_zone*/` — world-readable `type` and `temp` files, no subprocess) and renders one line per zone with its label, e.g. `52°C (x86_pkg_temp)`.
- Configurable unit via plugin args: `unit: "celsius"` (default) or `"fahrenheit"`.
- Windows support: WMI `MSAcpi_ThermalZoneTemperature` via `wmic` with a `powershell` fallback (same probe pattern as the core's battery/GPU detectors); WMI reports tenths of Kelvin, converted to the configured unit.
- macOS and other platforms report `Unsupported platform` (no portable world-readable sensor source yet).
- Standard info-plugin protocol (`xfetch_plugin_api`): usable from any xfetch config as `plugin:temperature` with custom icon/color.
