use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

const DEFAULT_REGISTRY: &str =
    "https://raw.githubusercontent.com/xfetch-cli/configs/main/themes/index.json";

#[derive(Debug, Deserialize)]
#[serde(default)]
struct PluginArgs {
    action: String,
    name: Option<String>,
    query: Option<String>,
    registry: Option<String>,
}

impl Default for PluginArgs {
    fn default() -> Self {
        Self {
            action: "list".to_string(),
            name: None,
            query: None,
            registry: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ThemeIndex {
    #[allow(dead_code)]
    registry: String,
    themes: Vec<ThemeEntry>,
}

#[derive(Debug, Deserialize)]
struct ThemeEntry {
    name: String,
    author: String,
    version: String,
    description: String,
    layout: Option<String>,
    palette_style: Option<String>,
    tags: Option<Vec<String>>,
    source: String,
}

fn main() {
    let args = match read_info_plugin_args_or_default::<PluginArgs>() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let lines = match handle_action(&args) {
        Ok(lines) => lines,
        Err(err) => vec![format!("Theme Manager: {}", err)],
    };

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn handle_action(args: &PluginArgs) -> Result<Vec<String>, String> {
    match args.action.as_str() {
        "list" => list_themes(args),
        "search" => search_themes(args),
        "info" => theme_info(args),
        "install" => install_theme(args),
        _ => Err(format!("Unknown action '{}'. Use list, search, info, or install.", args.action)),
    }
}

fn fetch_index(registry_url: &str) -> Result<ThemeIndex, String> {
    let data = if registry_url.starts_with('/') || registry_url.starts_with('~') {
        let path = if registry_url.starts_with('~') {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            registry_url.replacen('~', &home, 1)
        } else {
            registry_url.to_string()
        };
        std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read local registry '{}': {}", path, e))?
    } else {
        let output = Command::new("curl")
            .args(["-s", "--max-time", "15", registry_url])
            .output()
            .map_err(|e| format!("Failed to run curl: {}", e))?;

        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            return Err(format!(
                "Failed to fetch theme registry (HTTP {}). Check your internet connection.",
                code
            ));
        }

        String::from_utf8_lossy(&output.stdout).to_string()
    };

    serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse theme registry: {}", e))
}

fn theme_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("xfetch").join("themes")
}

fn list_themes(args: &PluginArgs) -> Result<Vec<String>, String> {
    let registry_url = args.registry.as_deref().unwrap_or(DEFAULT_REGISTRY);
    let index = fetch_index(registry_url)?;

    if index.themes.is_empty() {
        return Ok(vec!["Theme Manager: no themes available in registry".to_string()]);
    }

    let mut result = Vec::new();
    result.push(format!("Theme Manager -- {} themes available", index.themes.len()));
    result.push(String::new());

    for theme in &index.themes {
        let layout = theme.layout.as_deref().unwrap_or("default");
        let tags = theme
            .tags
            .as_ref()
            .map(|t| format!("#{}", t.join(" #")))
            .unwrap_or_default();
        result.push(format!("  {}  {}  {}  ({})", theme.name, theme.author, layout, tags));
        result.push(format!("       {}", theme.description));
        result.push(String::new());
    }

    Ok(result)
}

fn search_themes(args: &PluginArgs) -> Result<Vec<String>, String> {
    let registry_url = args.registry.as_deref().unwrap_or(DEFAULT_REGISTRY);
    let query = args.query.as_deref().unwrap_or("").to_lowercase();
    let index = fetch_index(registry_url)?;

    if query.is_empty() {
        return list_themes(args);
    }

    let matches: Vec<&ThemeEntry> = index
        .themes
        .iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&query)
                || t.description.to_lowercase().contains(&query)
                || t.author.to_lowercase().contains(&query)
                || t.tags
                    .as_ref()
                    .map(|tags| tags.iter().any(|tag| tag.to_lowercase().contains(&query)))
                    .unwrap_or(false)
        })
        .collect();

    if matches.is_empty() {
        return Ok(vec![format!("Theme Manager: no themes matching '{}'", query)]);
    }

    let mut result = Vec::new();
    result.push(format!(
        "Theme Manager -- {} themes matching '{}'",
        matches.len(),
        query
    ));
    result.push(String::new());

    for theme in matches {
        let layout = theme.layout.as_deref().unwrap_or("default");
        let tags = theme
            .tags
            .as_ref()
            .map(|t| format!("#{}", t.join(" #")))
            .unwrap_or_default();
        result.push(format!("  {}  {}  {}  ({})", theme.name, theme.author, layout, tags));
        result.push(format!("       {}", theme.description));
        result.push(String::new());
    }

    Ok(result)
}

fn theme_info(args: &PluginArgs) -> Result<Vec<String>, String> {
    let name = args
        .name
        .as_deref()
        .ok_or_else(|| "Theme name required for info action".to_string())?;
    let registry_url = args.registry.as_deref().unwrap_or(DEFAULT_REGISTRY);
    let index = fetch_index(registry_url)?;

    let theme = index
        .themes
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| format!("Theme '{}' not found in registry", name))?;

    let tags = theme
        .tags
        .as_ref()
        .map(|t| t.join(", "))
        .unwrap_or_else(|| "none".to_string());

    let installed_path = theme_dir().join(format!("{}.jsonc", name));
    let installed = if installed_path.exists() { "yes" } else { "no" };

    Ok(vec![
        format!("Theme: {}", theme.name),
        format!("  Author:     {}", theme.author),
        format!("  Version:    {}", theme.version),
        format!("  Layout:     {}", theme.layout.as_deref().unwrap_or("default")),
        format!("  Palette:    {}", theme.palette_style.as_deref().unwrap_or("default")),
        format!("  Tags:       {}", tags),
        format!("  Installed:  {}", installed),
        String::new(),
        format!("  {}", theme.description),
        String::new(),
        format!("  Source: {}", theme.source),
    ])
}

fn install_theme(args: &PluginArgs) -> Result<Vec<String>, String> {
    let name = args
        .name
        .as_deref()
        .ok_or_else(|| "Theme name required for install action".to_string())?;

    let registry_url = args.registry.as_deref().unwrap_or(DEFAULT_REGISTRY);
    let index = fetch_index(registry_url)?;

    let theme = index
        .themes
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| format!("Theme '{}' not found in registry", name))?;

    let dest_dir = theme_dir();
    std::fs::create_dir_all(&dest_dir)
        .map_err(|e| format!("Failed to create themes directory: {}", e))?;

    let dest_path = dest_dir.join(format!("{}.jsonc", name));

    let source = &theme.source;
    if source.starts_with('/') || source.starts_with('~') {
        let src_path = if source.starts_with('~') {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            source.replacen('~', &home, 1)
        } else {
            source.to_string()
        };
        std::fs::copy(&src_path, &dest_path)
            .map_err(|e| format!("Failed to copy theme file: {}", e))?;
    } else {
        let output = Command::new("curl")
            .args(["-s", "--max-time", "30", "-o", &dest_path.to_string_lossy(), source])
            .output()
            .map_err(|e| format!("Failed to run curl: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to download theme '{}': {}", name, stderr));
        }
    }

    if !dest_path.is_file() || dest_path.metadata().map(|m| m.len()).unwrap_or(0) == 0 {
        return Err(format!("Downloaded theme '{}' is empty or missing", name));
    }

    let layout = theme.layout.as_deref().unwrap_or("default");

    Ok(vec![
        format!("Theme '{}' installed successfully.", name),
        format!("  Path: {}", dest_path.display()),
        format!("  Author: {}", theme.author),
        format!("  Layout: {}", layout),
        String::new(),
        format!("Activate with: xfetch theme set {}", name),
        format!("Or add to config.jsonc: \"theme\": \"{}\"", name),
    ])
}
