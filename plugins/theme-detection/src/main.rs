#[cfg(not(target_os = "windows"))]
use std::fs;
#[cfg(not(target_os = "windows"))]
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {}

/// Local probes only (gsettings); 2 s is plenty.
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

        get_theme_info()
    })
    .unwrap_or_else(|_| vec!["Theme: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

/// Linux/macOS: GTK (gsettings) and KDE Plasma detection.
#[cfg(not(target_os = "windows"))]
fn get_theme_info() -> Vec<String> {
    let mut result = Vec::new();

    let gtk_theme = get_gsetting("org.gnome.desktop.interface", "gtk-theme");
    let icon_theme = get_gsetting("org.gnome.desktop.interface", "icon-theme");
    let cursor_theme = get_gsetting("org.gnome.desktop.interface", "cursor-theme");
    let font_name = get_gsetting("org.gnome.desktop.interface", "font-name");
    let color_scheme = get_gsetting("org.gnome.desktop.interface", "color-scheme");

    let kde_theme = get_kde_theme();
    let kde_color = get_kde_color_scheme();

    match (&gtk_theme, &kde_theme) {
        (Some(_), None) | (None, None) => {
            if let Some(ref theme) = gtk_theme {
                let variant = color_scheme.as_deref().unwrap_or("default");
                let icon = if variant.contains("dark") {
                    ""
                } else {
                    ""
                };
                result.push(format!(
                    "{} GTK Theme: {} ({})",
                    icon,
                    theme,
                    variant_display(variant)
                ));
            } else {
                result.push(" Theme: not detected".to_string());
                return result;
            }
        }
        (None, Some(_)) | (Some(_), Some(_)) => {
            if let Some(ref theme) = gtk_theme {
                let variant = color_scheme.as_deref().unwrap_or("default");
                let icon = if variant.contains("dark") {
                    ""
                } else {
                    ""
                };
                result.push(format!(
                    "{} GTK: {} ({})",
                    icon,
                    theme,
                    variant_display(variant)
                ));
            }
            if let Some(ref theme) = kde_theme {
                result.push(format!("   Plasma: {}", theme));
                if let Some(ref color) = kde_color {
                    result.push(format!("   Colors: {}", color));
                }
            }
        }
    }

    if let Some(ref icons) = icon_theme {
        result.push(format!("   Icons: {}", icons));
    }

    if let Some(ref cursor) = cursor_theme {
        result.push(format!("   Cursor: {}", cursor));
    }

    if let Some(ref font) = font_name {
        result.push(format!("   Font: {}", font));
    }

    result
}

/// Windows: light/dark mode and accent color from the registry.
#[cfg(target_os = "windows")]
fn get_theme_info() -> Vec<String> {
    get_windows_theme_info()
}

/// Windows: light/dark mode and accent color from the registry.
#[cfg(target_os = "windows")]
fn get_windows_theme_info() -> Vec<String> {
    let mut result = Vec::new();

    match get_reg_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
        "AppsUseLightTheme",
    ) {
        Some(1) => result.push("  Windows Theme: Light".to_string()),
        Some(0) => result.push("  Windows Theme: Dark".to_string()),
        _ => result.push("Theme: not detected".to_string()),
    }

    if let Some(color) = get_reg_dword(r"HKCU\Software\Microsoft\Windows\DWM", "ColorizationColor")
    {
        // DWM stores the color as AABBGGRR.
        let b = (color >> 16) & 0xFF;
        let g = (color >> 8) & 0xFF;
        let r = color & 0xFF;
        result.push(format!("  Accent: #{:02X}{:02X}{:02X}", r, g, b));
    }

    result
}

/// Reads a REG_DWORD value (hex, e.g. `0x1`) from the Windows registry.
#[cfg(target_os = "windows")]
fn get_reg_dword(subkey: &str, name: &str) -> Option<u32> {
    let output = Command::new("reg")
        .args(["query", subkey, "/v", name])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let out = String::from_utf8_lossy(&output.stdout);
    let value = out.lines().find_map(|line| {
        let line = line.trim();
        line.starts_with(name).then(|| {
            line.rsplit(' ')
                .next()
                .unwrap_or_default()
                .trim()
                .to_string()
        })
    })?;

    u32::from_str_radix(value.trim_start_matches("0x"), 16).ok()
}

#[cfg(not(target_os = "windows"))]
fn get_gsetting(schema: &str, key: &str) -> Option<String> {
    let output = Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() || value == "''" || value == "\"\"" {
        return None;
    }

    Some(value.trim_matches('\'').trim_matches('"').to_string())
}

#[cfg(not(target_os = "windows"))]
fn get_kde_theme() -> Option<String> {
    let config_paths = vec![
        format!(
            "{}/.config/plasmarc",
            std::env::var("HOME").unwrap_or_default()
        ),
        format!(
            "{}/.config/kdeglobals",
            std::env::var("HOME").unwrap_or_default()
        ),
    ];

    for path in config_paths {
        if Path::new(&path).exists()
            && let Ok(content) = fs::read_to_string(&path)
        {
            for line in content.lines() {
                if let Some(theme) = line.strip_prefix("theme=") {
                    return Some(theme.to_string());
                }
            }
        }
    }

    None
}

#[cfg(not(target_os = "windows"))]
fn get_kde_color_scheme() -> Option<String> {
    let config_path = format!(
        "{}/.config/kdeglobals",
        std::env::var("HOME").unwrap_or_default()
    );

    if !Path::new(&config_path).exists() {
        return None;
    }

    let content = fs::read_to_string(&config_path).ok()?;
    let mut in_general = false;

    for line in content.lines() {
        if line.trim() == "[General]" {
            in_general = true;
            continue;
        }
        if in_general {
            if line.starts_with('[') {
                break;
            }
            if let Some(scheme) = line.strip_prefix("ColorScheme=") {
                return Some(scheme.to_string());
            }
        }
    }

    None
}

#[cfg(not(target_os = "windows"))]
fn variant_display(variant: &str) -> &str {
    match variant {
        "prefer-dark" => "dark",
        "prefer-light" => "light",
        "default" => "default",
        _ => variant,
    }
}
