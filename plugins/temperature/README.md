# xfetch-plugin-temperature

CPU/SoC temperature module for [xfetch](https://github.com/xfetch-cli/xfetch).

Reads the kernel thermal zones (`/sys/class/thermal/thermal_zone*/` — world-readable files, no subprocess) on Linux. Other platforms report unsupported.

## Usage

Install the plugin binary in the xfetch plugin dir, then add it to your config:

```jsonc
{
    "info_plugins": [
        { "plugin": "temperature", "args": { "unit": "celsius" } }
    ],
    "modules": [ "plugin:temperature" ],
    "icons": { "plugin:temperature": "" }
}
```

`unit` accepts `"celsius"` (default) or `"fahrenheit"`.

## Platform support

| Platform | Source | Notes |
|---|---|---|
| Linux | `/sys/class/thermal/thermal_zone*/` | World-readable files, no subprocess. |
| Windows | WMI `MSAcpi_ThermalZoneTemperature` | Via `wmic`, `powershell` fallback; needs no admin on most machines. |
| macOS | — | Unsupported: the only real sources (`powermetrics`, SMC) require root or third-party tools. Reports `Unsupported platform`. |
