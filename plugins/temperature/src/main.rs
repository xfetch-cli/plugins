use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    /// "celsius" (default) or "fahrenheit"
    unit: Option<String>,
}

/// Local probes only (hwmon/sensors); 2 s is plenty.
const BUDGET: Duration = Duration::from_secs(2);

fn main() {
    let lines = with_timeout(BUDGET, || {
        let args = match read_info_plugin_args_or_default::<PluginArgs>() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        get_temperature_info(args.unit.as_deref())
    })
    .unwrap_or_else(|_| vec!["Temperature: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn format_temp(celsius: f64, unit: Option<&str>) -> String {
    let temp = celsius.max(0.0);
    if unit == Some("fahrenheit") {
        format!("{:.0}°F", temp * 9.0 / 5.0 + 32.0)
    } else {
        format!("{:.0}°C", temp)
    }
}

/// Kernel-exposed thermal zones. Linux: `/sys/class/thermal/thermal_zone*/`
/// (world-readable `type` + `temp` files, no subprocess). Windows: WMI
/// `MSAcpi_ThermalZoneTemperature` via `wmic` with a `powershell` fallback
/// (the same probe pattern as the core's battery/GPU detectors).
fn get_temperature_info(unit: Option<&str>) -> Vec<String> {
    let mut lines = Vec::new();
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/sys/class/thermal") {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if !name.starts_with("thermal_zone") {
                    continue;
                }
                let path = entry.path();
                let zone_type = std::fs::read_to_string(path.join("type"))
                    .ok()
                    .map(|s| s.trim().to_string());
                let Ok(raw) = std::fs::read_to_string(path.join("temp")) else {
                    continue;
                };
                let Ok(milli) = raw.trim().parse::<i64>() else {
                    continue;
                };
                let celsius = milli as f64 / 1000.0;
                let label = zone_type.unwrap_or_else(|| "thermal".to_string());
                lines.push(format!(" {} ({})", format_temp(celsius, unit), label));
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        lines.extend(get_windows_temperature(unit));
    }
    if lines.is_empty() {
        lines.push(" Unsupported platform".to_string());
    }
    lines
}

/// WMI reports `MSAcpi_ThermalZoneTemperature.CurrentTemperature` in tenths of
/// degrees Kelvin; the probe returns (zone name, tenths-of-kelvin) pairs.
#[cfg(target_os = "windows")]
fn run_windows_probe(cmd: &str, args: &[&str]) -> Vec<(String, i64)> {
    let Ok(output) = std::process::Command::new(cmd).args(args).output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut zones = Vec::new();
    for line in stdout.lines().skip(1) {
        let mut name_parts: Vec<&str> = Vec::new();
        let mut tenths = None;
        for token in line.split_whitespace() {
            if let Ok(t) = token.parse::<i64>() {
                tenths = Some(t);
            } else {
                name_parts.push(token);
            }
        }
        if let Some(t) = tenths {
            let name = if name_parts.is_empty() {
                "thermal".to_string()
            } else {
                name_parts.join(" ")
            };
            zones.push((name, t));
        }
    }
    zones
}

#[cfg(target_os = "windows")]
fn get_windows_temperature(unit: Option<&str>) -> Vec<String> {
    let zones = run_windows_probe(
        "wmic",
        &[
            "path",
            "MSAcpi_ThermalZoneTemperature",
            "get",
            "CurrentTemperature,InstanceName",
        ],
    );
    let zones = if zones.is_empty() {
        run_windows_probe(
            "powershell",
            &[
                "-Command",
                "Get-CimInstance -Namespace root/wmi -ClassName MSAcpi_ThermalZoneTemperature | ForEach-Object { \"$($_.CurrentTemperature) $($_.InstanceName)\" }",
            ],
        )
    } else {
        zones
    };
    zones
        .iter()
        .map(|(name, tenths)| {
            let celsius = *tenths as f64 / 10.0 - 273.15;
            format!(" {} ({})", format_temp(celsius, unit), name)
        })
        .collect()
}
