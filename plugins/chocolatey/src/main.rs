use std::process::Command;
use std::time::Duration;
use xfetch_plugin_api::{
    EmptyArgs, read_info_plugin_args_or_default, with_timeout, write_info_lines,
};

/// `choco list` can be slow on first runs (package cache, license checks).
const BUDGET: Duration = Duration::from_secs(10);

fn main() {
    let lines = with_timeout(BUDGET, || {
        let _args = match read_info_plugin_args_or_default::<EmptyArgs>() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        get_choco_info()
    })
    .unwrap_or_else(|_| vec!["Chocolatey: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

/// Counts locally installed Chocolatey packages via `choco list -r`
/// (`name|version` per line, no banner or summary line). Chocolatey is a
/// Windows-only tool; elsewhere the command is missing and a fallback line
/// is returned.
fn get_choco_info() -> Vec<String> {
    let output = match Command::new("choco").args(["list", "-r"]).output() {
        Ok(o) if o.status.success() => o,
        _ => return vec!["Chocolatey: not installed".to_string()],
    };

    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| {
            let l = l.trim();
            !l.is_empty() && l.contains('|')
        })
        .count();

    if count == 0 {
        return vec!["Chocolatey: no packages installed".to_string()];
    }

    vec![format!("\u{f187} {} ({})", count, "chocolatey")]
}
