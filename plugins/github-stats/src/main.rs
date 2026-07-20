use serde::Deserialize;
use std::process::Command;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, Deserialize)]
struct PluginArgs {
    username: Option<String>,
    token: Option<String>,
    max_lines: Option<usize>,
}

fn main() {
    let args = match read_info_plugin_args_or_default::<PluginArgs>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let username = args
        .username
        .clone()
        .or_else(|| std::env::var("GITHUB_USER").ok())
        .filter(|u| !u.is_empty());

    let max_lines = args.max_lines;

    let lines = match username {
        Some(user) => {
            let mut stats = get_github_stats(&user, args.token.as_deref());
            if let Some(limit) = max_lines {
                stats.truncate(limit);
            }
            stats
        }
        None => vec![" GitHub: no username configured".to_string()],
    };

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_github_stats(username: &str, token: Option<&str>) -> Vec<String> {
    let mut result = Vec::new();

    let api_url = format!("https://api.github.com/users/{}", username);
    let repos_url = format!("https://api.github.com/users/{}/repos?per_page=100&sort=pushed", username);
    let prs_url = format!("https://api.github.com/search/issues?q=author:{} type:pr&per_page=0", username);
    let issues_url = format!("https://api.github.com/search/issues?q=author:{} type:issue&per_page=0", username);

    let user_info = fetch_json(&api_url);
    let repos_info = fetch_json(&repos_url);
    let prs_info = fetch_json_with_token(&prs_url, token);
    let issues_info = fetch_json_with_token(&issues_url, token);

    match user_info {
        Ok(user) => {
            let login = user["login"].as_str().unwrap_or(username);
            let name = user["name"].as_str().unwrap_or(login);
            let public_repos = user["public_repos"].as_u64().unwrap_or(0);
            let followers = user["followers"].as_u64().unwrap_or(0);
            let following = user["following"].as_u64().unwrap_or(0);

            let stars = match repos_info {
                Ok(ref r) => r.as_array()
                    .map(|arr| arr.iter().filter_map(|repo| repo["stargazers_count"].as_u64()).sum())
                    .unwrap_or(0),
                Err(_) => 0,
            };

            let prs = prs_info.as_ref()
                .ok()
                .and_then(|r| r["total_count"].as_u64())
                .unwrap_or(0);

            let issues = issues_info.as_ref()
                .ok()
                .and_then(|r| r["total_count"].as_u64())
                .unwrap_or(0);

            result.push(format!(" {} (@{})", name, login));
            result.push(format!(" {} stars", stars));
            result.push(format!(" {} repos", public_repos));
            result.push(format!(" {} PRs", prs));
            result.push(format!(" {} issues", issues));
            result.push(format!(" {} followers", followers));
            result.push(format!(" {} following", following));
        }
        Err(_) => {
            result.push(format!(" GitHub: could not fetch user '{}'", username));
        }
    }

    result
}

fn fetch_json(url: &str) -> Result<serde_json::Value, String> {
    fetch_json_with_token(url, None)
}

fn fetch_json_with_token(url: &str, token: Option<&str>) -> Result<serde_json::Value, String> {
    let mut args = vec![
        "-s".to_string(),
        "-H".to_string(),
        "Accept: application/vnd.github.v3+json".to_string(),
        "-H".to_string(),
        "User-Agent: xfetch-plugin-github-stats/0.1".to_string(),
    ];

    if let Some(t) = token {
        args.push("-H".to_string());
        args.push(format!("Authorization: token {}", t));
    }

    args.push(url.to_string());

    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let output = Command::new("curl")
        .args(&args_refs)
        .output()
        .map_err(|e| format!("Failed to run curl: {}", e))?;

    if !output.status.success() {
        return Err("curl exited with error".to_string());
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse JSON: {}", e))
}
