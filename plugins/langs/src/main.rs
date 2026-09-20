//! Language breakdown plus repository/owner metadata for GitHub.
//!
//! Each line is `<glyph> <label> <value>`; the glyph lets the core pick it as
//! the row icon and the label can be tinted with a brand colour.

use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    /// Repository as `owner/name`.
    repo: Option<String>,
    /// User or org to aggregate the primary language of its repositories.
    owner: Option<String>,
    /// How many languages to show (default 5).
    limit: Option<usize>,
    /// Tint labels with their brand colour (default true).
    color: Option<bool>,
    /// Include forked repositories when aggregating an owner (default false).
    include_forks: Option<bool>,
    /// Include repository/owner metadata lines (default true).
    info: Option<bool>,
    /// GitHub token; also read from `GITHUB_TOKEN` / `GH_TOKEN`.
    token: Option<String>,
}

/// Network budget: two API calls.
const BUDGET: Duration = Duration::from_secs(20);

fn main() {
    let lines = with_timeout(BUDGET, || {
        let args = match read_info_plugin_args_or_default::<PluginArgs>() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        run(&args)
    })
    .unwrap_or_else(|_| vec![fallback("timed out")]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn fallback(message: &str) -> String {
    line("\u{f121}", "Languages", message, "#AAAAAA", true)
}

fn run(args: &PluginArgs) -> Vec<String> {
    let token = resolve_token(args.token.as_deref());
    let color = args.color.unwrap_or(true);
    let info = args.info.unwrap_or(true);
    let limit = args.limit.unwrap_or(5).max(1);

    if let Some(repo) = args.repo.as_deref() {
        return repo_report(repo, limit, color, info, token.as_deref());
    }
    if let Some(owner) = args.owner.as_deref() {
        return owner_report(
            owner,
            limit,
            color,
            info,
            args.include_forks.unwrap_or(false),
            token.as_deref(),
        );
    }
    vec![fallback("set `repo` or `owner`")]
}

/// Token precedence: `token` arg, `GITHUB_TOKEN`, `GH_TOKEN`, then the
/// `gh auth token` CLI (so a machine logged in with `gh` just works).
fn resolve_token(arg: Option<&str>) -> Option<String> {
    if let Some(token) = arg.filter(|token| !token.is_empty()) {
        return Some(token.to_string());
    }
    for var in ["GITHUB_TOKEN", "GH_TOKEN"] {
        if let Ok(token) = std::env::var(var)
            && !token.is_empty()
        {
            return Some(token);
        }
    }
    let output = Command::new("gh").args(["auth", "token"]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!token.is_empty()).then_some(token)
}

/// Repository metadata + exact byte breakdown from `/repos/{repo}/languages`.
fn repo_report(
    repo: &str,
    limit: usize,
    color: bool,
    info: bool,
    token: Option<&str>,
) -> Vec<String> {
    let slug = normalize_repo(repo);
    let mut lines = Vec::new();

    if info {
        match fetch_json(&format!("https://api.github.com/repos/{slug}"), token) {
            Ok(meta) => lines.extend(repo_info_lines(&meta, color)),
            Err(err) => return vec![fallback(&format!("{slug}: {err}"))],
        }
    }

    match fetch_json(
        &format!("https://api.github.com/repos/{slug}/languages"),
        token,
    ) {
        Ok(value) => {
            let Some(map) = value.as_object() else {
                lines.push(fallback("unexpected language response"));
                return lines;
            };
            let mut totals: Vec<(String, u64)> = map
                .iter()
                .filter_map(|(name, bytes)| bytes.as_u64().map(|bytes| (name.clone(), bytes)))
                .collect();
            let total: u64 = totals.iter().map(|(_, bytes)| bytes).sum();
            totals.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            for (name, bytes) in totals.into_iter().take(limit) {
                let share = if total == 0 {
                    0.0
                } else {
                    bytes as f64 * 100.0 / total as f64
                };
                let (glyph, hex) = meta(&name);
                lines.push(line(glyph, &name, &format!("{share:.1}%"), hex, color));
            }
        }
        Err(err) => lines.push(fallback(&err)),
    }
    lines
}

/// Owner summary + aggregated primary languages of its repositories.
fn owner_report(
    owner: &str,
    limit: usize,
    color: bool,
    info: bool,
    include_forks: bool,
    token: Option<&str>,
) -> Vec<String> {
    let owner = owner.trim().trim_end_matches('/');
    let mut lines = Vec::new();

    if info && let Ok(meta) = fetch_json(&format!("https://api.github.com/users/{owner}"), token) {
        if let Some(name) = meta["name"].as_str().filter(|name| !name.is_empty()) {
            lines.push(line("\u{f007}", name, "", "#7DCFFF", color));
        }
        if let Some(bio) = meta["bio"].as_str().filter(|bio| !bio.is_empty()) {
            lines.push(line(
                "\u{f05a}",
                "Bio",
                &truncate(bio, 70),
                "#9AA5CE",
                color,
            ));
        }
        if let Some(count) = meta["public_repos"].as_u64() {
            lines.push(line(
                "\u{f07b}",
                "Public repos",
                &count.to_string(),
                "#7AA2F7",
                color,
            ));
        }
        if let Some(count) = meta["followers"].as_u64() {
            lines.push(line(
                "\u{f0c0}",
                "Followers",
                &count.to_string(),
                "#BB9AF7",
                color,
            ));
        }
    }

    let mut repos = None;
    for url in [
        format!("https://api.github.com/orgs/{owner}/repos?per_page=100&page=1&sort=pushed"),
        format!("https://api.github.com/users/{owner}/repos?per_page=100&page=1&sort=pushed"),
    ] {
        if let Ok(value) = fetch_json(&url, token)
            && let Some(array) = value.as_array()
        {
            repos = Some(array.clone());
            break;
        }
    }
    let Some(repos) = repos else {
        lines.push(fallback(&format!("cannot list repositories for '{owner}'")));
        return lines;
    };

    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut scanned = 0u64;
    for repo in &repos {
        if !include_forks && repo["fork"].as_bool().unwrap_or(false) {
            continue;
        }
        if let Some(language) = repo["language"].as_str() {
            *counts.entry(language.to_string()).or_default() += 1;
            scanned += 1;
        }
    }
    if scanned == 0 {
        lines.push(fallback(&format!("no languages found for '{owner}'")));
        return lines;
    }

    let mut ranked: Vec<(String, u64)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for (name, count) in ranked.into_iter().take(limit) {
        let share = count as f64 * 100.0 / scanned as f64;
        let (glyph, hex) = meta(&name);
        lines.push(line(
            glyph,
            &name,
            &format!("×{count} ({share:.0}%)"),
            hex,
            color,
        ));
    }
    lines
}

fn repo_info_lines(meta: &serde_json::Value, color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(description) = meta["description"].as_str().filter(|d| !d.is_empty()) {
        lines.push(line(
            "\u{f05a}",
            "Description",
            &truncate(description, 70),
            "#9AA5CE",
            color,
        ));
    }
    if let Some(stars) = meta["stargazers_count"].as_u64() {
        lines.push(line(
            "\u{f005}",
            "Stars",
            &stars.to_string(),
            "#F9C513",
            color,
        ));
    }
    if let Some(forks) = meta["forks_count"].as_u64() {
        lines.push(line(
            "\u{f126}",
            "Forks",
            &forks.to_string(),
            "#8A8A8A",
            color,
        ));
    }
    if let Some(issues) = meta["open_issues_count"].as_u64() {
        lines.push(line(
            "\u{f188}",
            "Open issues",
            &issues.to_string(),
            "#E06C75",
            color,
        ));
    }
    if let Some(license) = meta["license"]["spdx_id"]
        .as_str()
        .filter(|l| *l != "NOASSERTION")
    {
        lines.push(line("\u{f2c2}", "License", license, "#A3BE8C", color));
    }
    if let Some(kb) = meta["size"].as_u64() {
        lines.push(line("\u{f0a0}", "Size", &human_size(kb), "#88C0D0", color));
    }
    if let Some(branch) = meta["default_branch"].as_str() {
        lines.push(line("\u{e725}", "Branch", branch, "#B48EAD", color));
    }
    if let Some(pushed) = meta["pushed_at"].as_str().and_then(|s| s.get(0..10)) {
        lines.push(line("\u{f017}", "Updated", pushed, "#D08770", color));
    }
    if let Some(topics) = meta["topics"].as_array().filter(|t| !t.is_empty()) {
        let topics: Vec<&str> = topics.iter().filter_map(|t| t.as_str()).take(6).collect();
        lines.push(line(
            "\u{f02b}",
            "Topics",
            &topics.join(", "),
            "#5E81AC",
            color,
        ));
    }
    if let Some(homepage) = meta["homepage"].as_str().filter(|h| !h.is_empty()) {
        lines.push(line("\u{f0ac}", "Homepage", homepage, "#81A1C1", color));
    }
    if meta["archived"].as_bool().unwrap_or(false) {
        lines.push(line("\u{f187}", "Archived", "yes", "#BF616A", color));
    }
    if meta["private"].as_bool().unwrap_or(false) {
        lines.push(line("\u{f023}", "Private", "yes", "#EBCB8B", color));
    }
    lines
}

/// `<glyph> tinted(label) value`; empty value prints only the label.
fn line(glyph: &str, label: &str, value: &str, hex: &str, color: bool) -> String {
    let label = tint(label, hex, color);
    if value.is_empty() {
        format!("{glyph} {label}")
    } else {
        format!("{glyph} {label} {value}")
    }
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn human_size(kb: u64) -> String {
    if kb >= 1024 * 1024 {
        format!("{:.1} GiB", kb as f64 / (1024.0 * 1024.0))
    } else if kb >= 1024 {
        format!("{:.1} MiB", kb as f64 / 1024.0)
    } else {
        format!("{kb} KiB")
    }
}

fn normalize_repo(repo: &str) -> String {
    let repo = repo.trim();
    let repo = repo.strip_prefix("https://github.com/").unwrap_or(repo);
    let repo = repo.strip_prefix("http://github.com/").unwrap_or(repo);
    repo.strip_suffix(".git")
        .unwrap_or(repo)
        .trim_matches('/')
        .to_string()
}

fn fetch_json(url: &str, token: Option<&str>) -> Result<serde_json::Value, String> {
    let mut args = vec![
        "-fsS".to_string(),
        "--max-time".to_string(),
        "15".to_string(),
        "-H".to_string(),
        "Accept: application/vnd.github+json".to_string(),
        "-H".to_string(),
        "User-Agent: xfetch-plugin-langs/0.1".to_string(),
    ];
    if let Some(token) = token {
        args.push("-H".to_string());
        args.push(format!("Authorization: Bearer {token}"));
    }
    args.push(url.to_string());

    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output = Command::new("curl")
        .args(&refs)
        .output()
        .map_err(|err| format!("curl failed: {err}"))?;
    if !output.status.success() {
        return Err("GitHub API request failed (rate limit?)".to_string());
    }
    serde_json::from_slice(&output.stdout).map_err(|_| "invalid API response".to_string())
}

/// `(glyph, brand colour)` for a language; falls back to a generic code glyph.
fn meta(language: &str) -> (&'static str, &'static str) {
    match language.to_ascii_lowercase().as_str() {
        "rust" => ("\u{e7a8}", "#DEA584"),
        "python" => ("\u{e73c}", "#3572A5"),
        "javascript" => ("\u{e74e}", "#F1E05A"),
        "typescript" => ("\u{e628}", "#3178C6"),
        "go" => ("\u{e627}", "#00ADD8"),
        "c" => ("\u{e61e}", "#555555"),
        "c++" => ("\u{e61d}", "#F34B7D"),
        "c#" => ("\u{e648}", "#178600"),
        "java" => ("\u{e738}", "#B07219"),
        "kotlin" => ("\u{e634}", "#A97BFF"),
        "swift" => ("\u{e755}", "#F05138"),
        "dart" => ("\u{e798}", "#00B4AB"),
        "html" => ("\u{e736}", "#E34C26"),
        "css" | "scss" | "sass" | "less" => ("\u{e749}", "#563D7C"),
        "shell" | "bash" | "zsh" | "fish" => ("\u{e795}", "#89E051"),
        "powershell" => ("\u{e795}", "#5391FE"),
        "batchfile" => ("\u{e795}", "#C1F12E"),
        "ruby" => ("\u{e739}", "#701516"),
        "php" => ("\u{e73d}", "#4F5D95"),
        "lua" => ("\u{e620}", "#51A0D5"),
        "perl" => ("\u{e769}", "#0298C3"),
        "r" => ("\u{f25d}", "#198CE7"),
        "scala" => ("\u{e737}", "#C22D40"),
        "elixir" => ("\u{e62d}", "#6E4A7E"),
        "erlang" => ("\u{e7b1}", "#B83998"),
        "haskell" => ("\u{e777}", "#5E5086"),
        "clojure" => ("\u{e768}", "#DB5855"),
        "ocaml" => ("\u{e67a}", "#3BE133"),
        "julia" => ("\u{e624}", "#A270BA"),
        "zig" => ("\u{e6a9}", "#EC915C"),
        "nim" => ("\u{e677}", "#FFC200"),
        "nix" => ("\u{f313}", "#7EBAE4"),
        "dockerfile" => ("\u{f308}", "#2496ED"),
        "makefile" | "cmake" => ("\u{e673}", "#427819"),
        "assembly" => ("\u{e637}", "#6E4C13"),
        "markdown" => ("\u{e73e}", "#083FA1"),
        "vue" => ("\u{e6a0}", "#41B883"),
        "svelte" => ("\u{e697}", "#FF3E00"),
        "objective-c" => ("\u{e61e}", "#438EFF"),
        "tex" => ("\u{e69b}", "#3D6117"),
        "vim script" => ("\u{e7c5}", "#199F4B"),
        "jupyter notebook" => ("\u{e678}", "#DA5B0B"),
        "groovy" => ("\u{e775}", "#4298B8"),
        "crystal" => ("\u{e62f}", "#5E8CBF"),
        _ => ("\u{f121}", "#AAAAAA"),
    }
}

fn tint(name: &str, hex: &str, enabled: bool) -> String {
    if !enabled {
        return name.to_string();
    }
    let Some((r, g, b)) = parse_hex(hex) else {
        return name.to_string();
    };
    format!("\x1b[38;2;{r};{g};{b}m{name}\x1b[0m")
}

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    Some((
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_token_wins() {
        assert_eq!(resolve_token(Some("abc")).as_deref(), Some("abc"));
        assert_ne!(resolve_token(Some("")).as_deref(), Some(""));
    }

    #[test]
    fn normalizes_repo_urls() {
        assert_eq!(normalize_repo("xfetch-cli/xfetch"), "xfetch-cli/xfetch");
        assert_eq!(
            normalize_repo("https://github.com/xfetch-cli/xfetch.git"),
            "xfetch-cli/xfetch"
        );
        assert_eq!(normalize_repo("/xfetch-cli/xfetch/"), "xfetch-cli/xfetch");
    }

    #[test]
    fn parses_hex_colors() {
        assert_eq!(parse_hex("#DEA584"), Some((222, 165, 132)));
        assert_eq!(parse_hex("DEA584"), None);
        assert_eq!(parse_hex("#GG0000"), None);
    }

    #[test]
    fn known_languages_get_glyphs() {
        assert_eq!(meta("Rust").0, "\u{e7a8}");
        assert_eq!(meta("PYTHON").1, "#3572A5");
        assert_eq!(meta("Brainfuck").0, "\u{f121}");
    }

    #[test]
    fn tint_can_be_disabled() {
        assert_eq!(tint("Rust", "#DEA584", false), "Rust");
        assert!(tint("Rust", "#DEA584", true).contains("38;2;222;165;132"));
    }

    #[test]
    fn human_size_units() {
        assert_eq!(human_size(512), "512 KiB");
        assert_eq!(human_size(2048), "2.0 MiB");
        assert_eq!(human_size(3 * 1024 * 1024), "3.0 GiB");
    }

    #[test]
    fn truncate_adds_ellipsis() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "hell…");
    }

    #[test]
    fn line_skips_empty_value() {
        assert_eq!(line("*", "Stars", "", "#FFFFFF", false), "* Stars");
        assert_eq!(line("*", "Stars", "3", "#FFFFFF", false), "* Stars 3");
    }
}
