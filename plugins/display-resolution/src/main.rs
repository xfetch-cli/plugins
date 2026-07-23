use std::process::Command;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {}

fn main() {
    let _args = match read_info_plugin_args_or_default::<PluginArgs>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let lines = get_display_info();

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_display_info() -> Vec<String> {
    if cfg!(target_os = "linux") {
        get_x11_display_info()
            .or_else(|| get_wayland_display_info())
            .or_else(|| get_xrandr_display_info())
            .unwrap_or_else(|| vec![" Display: unknown".to_string()])
    } else if cfg!(target_os = "macos") {
        get_macos_display_info()
            .unwrap_or_else(|| vec![" Display: unknown".to_string()])
    } else if cfg!(target_os = "windows") {
        get_windows_display_info()
            .unwrap_or_else(|| vec![" Display: unknown".to_string()])
    } else {
        vec![" Display: unsupported platform".to_string()]
    }
}

fn get_xrandr_display_info() -> Option<Vec<String>> {
    let output = Command::new("xrandr")
        .args(["--current"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = Vec::new();
    let mut display_count = 0;

    for line in stdout.lines() {
        let line = line.trim();
        if line.contains(" connected ") && line.contains('x') {
            display_count += 1;
            if let Some(res) = extract_resolution_xrandr(line) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let name = parts[0];
                let primary = if line.contains("primary") { " (primary)" } else { "" };
                if display_count == 1 {
                    result.push(format!(" {}: {}{}", name, res, primary));
                } else {
                    result.push(format!("   {}: {}{}", name, res, primary));
                }
            }
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn extract_resolution_xrandr(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if part.contains('x') && part.chars().all(|c| c.is_ascii_digit() || c == 'x') {
            if let Some(ref_rate) = parts.get(i + 1) {
                if ref_rate.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '*') {
                    let rate = ref_rate.trim_end_matches('*').trim_end_matches('+');
                    return Some(format!("{} @ {} Hz", part, rate));
                }
            }
            return Some(part.to_string());
        }
    }
    None
}

fn get_x11_display_info() -> Option<Vec<String>> {
    let output = Command::new("xdpyinfo")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("dimensions:") {
            let res = line.trim_start_matches("dimensions:").trim();
            result.push(format!(" Display: {}", res));
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn get_wayland_display_info() -> Option<Vec<String>> {
    let output = Command::new("wlr-randr")
        .output()
        .ok()
        .filter(|o| o.status.success())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = Vec::new();
    let mut current_name = String::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.ends_with(':') && !line.starts_with(' ') {
            current_name = line.trim_end_matches(':').to_string();
        }
        if let Some(resolution) = line.strip_suffix(" px") {
            if resolution.contains('x') && resolution.chars().all(|c| c.is_ascii_digit() || c == 'x') {
                let name = current_name.clone();
                if result.is_empty() {
                    result.push(format!(" {}: {} px", name, resolution));
                } else {
                    result.push(format!("   {}: {} px", name, resolution));
                }
            }
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn get_macos_display_info() -> Option<Vec<String>> {
    let output = Command::new("system_profiler")
        .args(["SPDisplaysDataType"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if let Some(res) = line.strip_prefix("Resolution: ") {
            result.push(format!(" Display: {}", res));
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn get_windows_display_info() -> Option<Vec<String>> {
    let ps_script = r#"
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class Screen {
    [DllImport("user32.dll")]
    public static extern IntPtr GetDC(IntPtr hwnd);
    [DllImport("gdi32.dll")]
    public static extern int GetDeviceCaps(IntPtr hdc, int nIndex);
    public const int HORZRES = 8;
    public const int VERTRES = 10;
}
"@
$hdc = [Screen]::GetDC([IntPtr]::Zero)
$w = [Screen]::GetDeviceCaps($hdc, [Screen]::HORZRES)
$h = [Screen]::GetDeviceCaps($hdc, [Screen]::VERTRES)
Write-Output "${w}x${h}"
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let res = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if res.is_empty() || !res.contains('x') {
        return None;
    }

    Some(vec![format!(" Display: {}", res)])
}
