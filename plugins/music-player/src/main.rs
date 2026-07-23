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

    let lines = get_music_info();

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_music_info() -> Vec<String> {
    let mut result = Vec::new();

    let mpd = get_mpd_status();
    let spotify = get_spotify_status();

    match (mpd, spotify) {
        (Some(m), Some(s)) => {
            result.push(" Music Players:".to_string());
            result.extend(m.iter().map(|l| format!("  {}", l)));
            result.extend(s.iter().map(|l| format!("  {}", l)));
        }
        (Some(m), None) => result.extend(m),
        (None, Some(s)) => result.extend(s),
        (None, None) => result.push(" Music: no active player".to_string()),
    }

    result
}

fn get_mpd_status() -> Option<Vec<String>> {
    let output = Command::new("mpc").args(["status"]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        return None;
    }

    let mut result = Vec::new();

    let song = lines.first()?.trim().to_string();
    if !song.is_empty() {
        result.push(format!(" MPD: {}", song));
    } else {
        result.push(" MPD: stopped".to_string());
    }

    if lines.len() > 1 {
        let status_line = lines[1].trim();
        if let Some(state) = status_line.split_whitespace().next() {
            let state_icon = match state {
                "[playing]" => "▶",
                "[paused]" => "⏸",
                _ => "⏹",
            };
            result.push(format!("  {} {}", state_icon, status_line));
        }
    }

    Some(result)
}

fn get_spotify_status() -> Option<Vec<String>> {
    let output = Command::new("playerctl")
        .args(["-p", "spotify", "metadata", "--format", "{{artist}} - {{title}}"])
        .output()
        .ok()?;

    if !output.status.success() {
        let paused = Command::new("playerctl")
            .args(["-p", "spotify", "status"])
            .output()
            .ok()?;
        if paused.status.success() {
            let status = String::from_utf8_lossy(&paused.stdout).trim().to_string();
            if status == "Paused" {
                return Some(vec![
                    " Spotify: paused".to_string(),
                ]);
            }
        }
        return None;
    }

    let meta = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if meta.is_empty() {
        return None;
    }

    Some(vec![
        format!(" Spotify: {}", meta),
    ])
}
