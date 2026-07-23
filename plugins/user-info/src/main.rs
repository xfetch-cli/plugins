use std::env;
use std::process::Command;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    show_groups: Option<bool>,
}

fn main() {
    let args = match read_info_plugin_args_or_default::<PluginArgs>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let lines = get_user_info(args.show_groups.unwrap_or(false));

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_user_info(show_groups: bool) -> Vec<String> {
    let mut result = Vec::new();

    let username = whoami();
    let uid = get_uid();
    let gid = get_gid();
    let home = get_home();
    let shell = get_shell();
    let gecos = get_gecos(&username);

    let display_name = gecos.as_deref().unwrap_or(&username);
    result.push(format!(" {} ({})", display_name, username));

    result.push(format!("   uid: {}  gid: {}", uid, gid));

    if let Some(h) = home {
        result.push(format!("   {}", h));
    }

    if let Some(s) = shell {
        let sname = s.rsplit('/').next().unwrap_or(&s);
        result.push(format!("   {}", sname));
    }

    if show_groups {
        let groups = get_groups();
        if !groups.is_empty() {
            let group_str = groups.join(", ");
            result.push(format!("   groups: {}", group_str));
        }
    }

    result
}

fn whoami() -> String {
    env::var("USER")
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| {
            Command::new("whoami")
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        })
}

fn get_uid() -> String {
    Command::new("id")
        .args(["-u"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "?".to_string())
}

fn get_gid() -> String {
    Command::new("id")
        .args(["-g"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "?".to_string())
}

fn get_home() -> Option<String> {
    env::var("HOME").ok()
}

fn get_shell() -> Option<String> {
    env::var("SHELL").ok()
}

fn get_gecos(username: &str) -> Option<String> {
    let output = Command::new("getent")
        .args(["passwd", username])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let entry = String::from_utf8_lossy(&output.stdout);
    let fields: Vec<&str> = entry.trim().split(':').collect();
    if fields.len() >= 5 {
        let gecos = fields[4].trim().to_string();
        if !gecos.is_empty() && gecos != "," {
            let name = gecos.split(',').next().unwrap_or("").trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    None
}

fn get_groups() -> Vec<String> {
    let output = match Command::new("groups").output() {
        Ok(o) if o.status.success() => o,
        _ => return Vec::new(),
    };

    let line = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let parts: Vec<&str> = line.split(':').collect();

    let groups_str = if parts.len() >= 2 {
        parts[1].trim()
    } else {
        parts[0].trim()
    };

    let current_user = whoami();
    groups_str
        .split_whitespace()
        .map(|s| s.to_string())
        .filter(|g| g != &current_user)
        .take(10)
        .collect()
}
