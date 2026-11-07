//! Emits text lines from a file or a literal argument as an info provider.
//!
//! Unlike logo animation plugins, info providers run even when xfetch renders
//! an image logo, which makes this plugin suitable for captions or banners
//! placed below the stats column.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    path: Option<String>,
    text: Option<String>,
    blank_line: Option<bool>,
}

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
        let raw = resolve_text(&args);
        normalize_lines(&raw, args.blank_line.unwrap_or(true))
    })
    .unwrap_or_default();

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn resolve_text(args: &PluginArgs) -> String {
    if let Some(path) = args
        .path
        .as_deref()
        .map(str::trim)
        .filter(|p| !p.is_empty())
        && let Ok(text) = fs::read_to_string(expand_path(path))
        && !text.trim().is_empty()
    {
        return text;
    }
    args.text.clone().unwrap_or_default()
}

fn normalize_lines(raw: &str, blank_line: bool) -> Vec<String> {
    let mut lines: Vec<String> = raw
        .lines()
        .map(|line| line.trim_end().to_string())
        .collect();
    while lines.first().is_some_and(|line| line.is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
    if blank_line && !lines.is_empty() {
        lines.insert(0, String::new());
    }
    lines
}

fn expand_path(path: &str) -> PathBuf {
    if path == "~" {
        return home_dir().unwrap_or_else(|| PathBuf::from(path));
    }
    if let Some(rest) = path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\"))
        && let Some(home) = home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(path)
}

fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    let key = "USERPROFILE";
    #[cfg(not(windows))]
    let key = "HOME";
    std::env::var_os(key).map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::normalize_lines;

    #[test]
    fn trims_trailing_whitespace_and_edge_blank_lines() {
        let lines = normalize_lines("\n  first  \n\nsecond\n\n", false);
        assert_eq!(lines, vec!["  first", "", "second"]);
    }

    #[test]
    fn prepends_one_blank_line_when_requested() {
        let lines = normalize_lines("banner", true);
        assert_eq!(lines, vec!["", "banner"]);
    }

    #[test]
    fn empty_text_returns_no_lines() {
        assert!(normalize_lines(" \n\t\n", true).is_empty());
    }
}
