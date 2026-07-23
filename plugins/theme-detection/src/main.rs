use std::fs;
use std::path::Path;
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

    let lines = get_theme_info();

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

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
                let icon = if variant.contains("dark") { "" } else { "" };
                result.push(format!("{} GTK Theme: {} ({})", icon, theme, variant_display(variant)));
            } else {
                result.push(" Theme: not detected".to_string());
                return result;
            }
        }
        (None, Some(_)) | (Some(_), Some(_)) => {
            if let Some(ref theme) = gtk_theme {
                let variant = color_scheme.as_deref().unwrap_or("default");
                let icon = if variant.contains("dark") { "" } else { "" };
                result.push(format!("{} GTK: {} ({})", icon, theme, variant_display(variant)));
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

    Some(
        value
            .trim_matches('\'')
            .trim_matches('"')
            .to_string(),
    )
}

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
        if Path::new(&path).exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    if line.starts_with("theme=") {
                        return Some(line[6..].to_string());
                    }
                }
            }
        }
    }

    None
}

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
            if line.starts_with("ColorScheme=") {
                return Some(line[12..].to_string());
            }
        }
    }

    None
}

fn variant_display(variant: &str) -> &str {
    match variant {
        "prefer-dark" => "dark",
        "prefer-light" => "light",
        "default" => "default",
        _ => variant,
    }
}
