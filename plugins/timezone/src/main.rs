#[cfg(not(target_os = "windows"))]
use std::fs;
#[cfg(not(target_os = "windows"))]
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    format: Option<String>,
}

/// Local probes only (date/timedatectl); 2 s is plenty.
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

        let fmt = args.format.as_deref().unwrap_or("%Z %z");
        get_tz_info(fmt)
    })
    .unwrap_or_else(|_| vec!["Timezone: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

/// Linux/macOS: `/etc/timezone`, `/etc/localtime`, `timedatectl` + GNU `date`.
#[cfg(not(target_os = "windows"))]
fn get_tz_info(format: &str) -> Vec<String> {
    let tz = detect_timezone();
    let tz_name = tz.as_deref().unwrap_or("unknown");

    let now_output = Command::new("date")
        .args([&format!("+{}", format)])
        .output();

    let date_info = match now_output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => String::new(),
    };

    let utc_offset = Command::new("date")
        .args(["+%:z"])
        .output();

    let offset = match utc_offset {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => String::new(),
    };

    let local_time = Command::new("date")
        .args(["+%A, %d %B %Y  %H:%M"])
        .output();

    let datetime = match local_time {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => String::new(),
    };

    let mut result = Vec::new();

    if !datetime.is_empty() {
        result.push(format!(" {}", datetime));
    }

    if !tz_name.is_empty() && tz_name != "unknown" {
        let display = tz_name.replace('_', " ");
        if !date_info.is_empty() {
            result.push(format!("   {} ({})", display, date_info));
        } else if !offset.is_empty() {
            result.push(format!("   {} ({})", display, offset));
        } else {
            result.push(format!("   {}", display));
        }
    } else if !offset.is_empty() {
        result.push(format!("   UTC offset: {}", offset));
    }

    if result.is_empty() {
        result.push(" Timezone: unknown".to_string());
    }

    result
}

/// Windows: `Get-TimeZone` + `Get-Date` via PowerShell.
///
/// The GNU `date` format string has no Windows equivalent, so the `format`
/// arg is ignored here (it only applies on Linux/macOS).
#[cfg(target_os = "windows")]
fn get_tz_info(_format: &str) -> Vec<String> {
    let mut result = Vec::new();

    if let Some(dt) = run_windows_cmd(&[
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        "[Console]::OutputEncoding=[Text.Encoding]::UTF8; Get-Date -Format 'dddd, dd MMMM yyyy  HH:mm'",
    ]) {
        result.push(format!("{} {}", "\u{f43a}", dt));
    }

    if let Some(tz) = run_windows_cmd(&[
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        "[Console]::OutputEncoding=[Text.Encoding]::UTF8; $z=Get-TimeZone; $d=$z.BaseUtcOffset.Duration(); $s=if($z.BaseUtcOffset.Ticks -lt 0){'-'}else{'+'}; \"$($z.Id) ($s$($d.ToString('hh\\:mm')))\"",
    ]) {
        result.push(format!("  {} {}", "\u{f2f2}", tz));
    }

    if result.is_empty() {
        result.push("Timezone: unknown".to_string());
    }

    result
}

#[cfg(target_os = "windows")]
fn run_windows_cmd(args: &[&str]) -> Option<String> {
    let output = Command::new("powershell").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[cfg(not(target_os = "windows"))]
fn detect_timezone() -> Option<String> {
    let tz_file = Path::new("/etc/timezone");
    if tz_file.exists() {
        if let Ok(content) = fs::read_to_string(tz_file) {
            let line = content.trim().to_string();
            if !line.is_empty() {
                return Some(line);
            }
        }
    }

    let localtime = Path::new("/etc/localtime");
    if localtime.exists() {
        if let Ok(path) = fs::read_link(localtime) {
            let path_str = path.to_string_lossy();
            if let Some(idx) = path_str.find("zoneinfo/") {
                let tz = path_str[idx + 9..].to_string();
                if !tz.is_empty() {
                    return Some(tz);
                }
            }
        }
    }

    let output = Command::new("timedatectl")
        .args(["show", "--property=Timezone", "--value"])
        .output()
        .ok()?;

    if output.status.success() {
        let tz = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !tz.is_empty() {
            return Some(tz);
        }
    }

    None
}
