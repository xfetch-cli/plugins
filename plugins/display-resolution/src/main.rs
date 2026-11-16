use std::process::Command;
use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {}

/// Local probes only (xrandr/wayland); 2 s is plenty.
const BUDGET: Duration = Duration::from_secs(2);

fn main() {
    let lines = with_timeout(BUDGET, || {
        let _args = match read_info_plugin_args_or_default::<PluginArgs>() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        get_display_info()
    })
    .unwrap_or_else(|_| vec!["Display: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_display_info() -> Vec<String> {
    if cfg!(target_os = "linux") {
        get_x11_display_info()
            .or_else(get_wayland_display_info)
            .or_else(get_xrandr_display_info)
            .unwrap_or_else(|| vec![" Display: unknown".to_string()])
    } else if cfg!(target_os = "macos") {
        get_macos_display_info().unwrap_or_else(|| vec![" Display: unknown".to_string()])
    } else if cfg!(target_os = "windows") {
        get_windows_display_info().unwrap_or_else(|| vec![" Display: unknown".to_string()])
    } else {
        vec![" Display: unsupported platform".to_string()]
    }
}

fn get_xrandr_display_info() -> Option<Vec<String>> {
    let output = Command::new("xrandr").args(["--current"]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = Vec::new();
    let mut display_count = 0;

    for line in stdout.lines() {
        let line = line.trim();
        if line.contains(" connected ")
            && let Some(res) = extract_resolution_xrandr(line)
        {
            display_count += 1;
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[0];
            let primary = if line.contains("primary") {
                " (primary)"
            } else {
                ""
            };
            if display_count == 1 {
                result.push(format!(" {}: {}{}", name, res, primary));
            } else {
                result.push(format!("   {}: {}{}", name, res, primary));
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn extract_resolution_xrandr(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if let Some(resolution) = resolution_from_xrandr_token(part) {
            if let Some(rate) = parts
                .get(i + 1)
                .and_then(|token| rate_from_xrandr_token(token))
            {
                return Some(format!("{resolution} @ {rate} Hz"));
            }
            return Some(resolution.to_string());
        }
    }
    None
}

/// Extracts the resolution from an xrandr geometry token.
///
/// `1920x1200+0+0` becomes `1920x1200`. The bare `x` token from the
/// `(normal left inverted right x axis y axis)` suffix is not a resolution.
fn resolution_from_xrandr_token(token: &str) -> Option<&str> {
    let end = token
        .find(|c: char| !(c.is_ascii_digit() || c == 'x'))
        .unwrap_or(token.len());
    let resolution = &token[..end];
    let (width, height) = resolution.split_once('x')?;
    if width.len() < 2
        || height.len() < 2
        || !width.chars().all(|c| c.is_ascii_digit())
        || !height.chars().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    Some(resolution)
}

fn rate_from_xrandr_token(token: &str) -> Option<&str> {
    let rate = token.trim_end_matches(['*', '+']);
    if rate.is_empty()
        || !rate.chars().all(|c| c.is_ascii_digit() || c == '.')
        || !rate.contains('.')
    {
        return None;
    }
    Some(rate)
}

fn get_x11_display_info() -> Option<Vec<String>> {
    let output = Command::new("xdpyinfo").output().ok()?;

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

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
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
        if let Some(resolution) = line.strip_suffix(" px")
            && resolution.contains('x')
            && resolution.chars().all(|c| c.is_ascii_digit() || c == 'x')
        {
            let name = current_name.clone();
            if result.is_empty() {
                result.push(format!(" {}: {} px", name, resolution));
            } else {
                result.push(format!("   {}: {} px", name, resolution));
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
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

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
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

#[cfg(test)]
mod tests {
    use super::{extract_resolution_xrandr, resolution_from_xrandr_token};

    #[test]
    fn xrandr_geometry_offset_is_not_part_of_the_resolution() {
        assert_eq!(
            extract_resolution_xrandr(
                "eDP-1 connected 1920x1200+0+0 (normal left inverted right x axis y axis) 340mm x 220mm"
            )
            .as_deref(),
            Some("1920x1200")
        );
    }

    #[test]
    fn xrandr_primary_display_ignores_auto_position() {
        assert_eq!(
            extract_resolution_xrandr(
                "HDMI-1 connected primary 3840x2160+1920+0 (normal left inverted right x axis y axis) 600mm x 340mm"
            )
            .as_deref(),
            Some("3840x2160")
        );
    }

    #[test]
    fn xrandr_prefers_explicit_refresh_rate() {
        assert_eq!(
            extract_resolution_xrandr("DP-1 connected 2560x1440 144.00*+").as_deref(),
            Some("2560x1440 @ 144.00 Hz")
        );
    }

    #[test]
    fn xrandr_interlaced_mode_is_accepted() {
        assert_eq!(
            extract_resolution_xrandr("DP-2 connected 1920x1080i+0+0").as_deref(),
            Some("1920x1080")
        );
    }

    #[test]
    fn bare_x_token_is_not_a_resolution() {
        assert_eq!(resolution_from_xrandr_token("x"), None);
        assert_eq!(resolution_from_xrandr_token("axis"), None);
    }
}
