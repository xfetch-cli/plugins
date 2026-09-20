//! Progress-bar gauges for memory, swap, disks and battery.
//!
//! Same measurements as the native core modules, but every line adds a text
//! bar next to the amount, e.g. `[////______] 10.96 GiB / 93.89 GiB (12%)`.

use std::time::Duration;
use sysinfo::{Disks, MemoryRefreshKind, RefreshKind, System};
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    /// Bar style: `auto` (default), `slashes`, `hash` or `blocks`.
    style: Option<String>,
    /// Bar width in characters (default 10, clamped to 3..=40).
    width: Option<usize>,
    /// Metrics to print; defaults to all (`memory`, `swap`, `disk`, `battery`).
    metrics: Option<Vec<String>>,
    /// Also list tmpfs/ramdisk mounts in the disk section.
    include_tmpfs: Option<bool>,
    /// Icon set: `nerd` (default), `cjk` (Chinese) or `jp` (Japanese).
    glyphs: Option<String>,
}

/// Icon set for the four metrics.
type Glyphs = (&'static str, &'static str, &'static str, &'static str);

fn glyphs(mode: &str) -> Glyphs {
    match mode {
        "cjk" => ("存", "换", "盘", "电"),
        "jp" => ("存", "換", "盤", "電"),
        _ => (GLYPH_MEM, GLYPH_SWAP, GLYPH_DISK, GLYPH_BATT),
    }
}

/// Local probes only; 3 s is plenty even with several mounts.
const BUDGET: Duration = Duration::from_secs(3);

/// Nerd Font glyphs prefixed to every line. The core lifts the leading
/// private-use glyph as the module icon, so the rows match the native ones
/// instead of falling back to the border-less `│ value` form.
const GLYPH_MEM: &str = "\u{e266}";
const GLYPH_SWAP: &str = "\u{f04e1}";
const GLYPH_DISK: &str = "\u{f02ca}";
const GLYPH_BATT: &str = "\u{f240}";

/// Filesystems that are not real storage and would only add noise.
const PSEUDO_FS: &[&str] = &[
    "proc",
    "sysfs",
    "devtmpfs",
    "devpts",
    "cgroup",
    "cgroup2",
    "squashfs",
    "overlay",
    "autofs",
    "mqueue",
    "debugfs",
    "tracefs",
    "securityfs",
    "pstore",
    "bpf",
    "configfs",
    "fusectl",
    "hugetlbfs",
    "binfmt_misc",
    "nsfs",
    "efivarfs",
    "ramfs",
    "fuse.gvfsd-fuse",
    "fuse.portal",
];

fn main() {
    let lines = with_timeout(BUDGET, || {
        let args = match read_info_plugin_args_or_default::<PluginArgs>() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        collect(&args)
    })
    .unwrap_or_else(|_| vec!["Gauges: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn collect(args: &PluginArgs) -> Vec<String> {
    let style = args.style.as_deref().unwrap_or("auto");
    let width = args.width.unwrap_or(10);
    let (g_mem, g_swap, g_disk, g_batt) = glyphs(args.glyphs.as_deref().unwrap_or("nerd"));
    let enabled = |name: &str| {
        args.metrics
            .as_ref()
            .is_none_or(|list| list.iter().any(|entry| entry == name))
    };

    let mut lines = Vec::new();

    if enabled("memory") || enabled("swap") {
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );
        if enabled("memory") {
            lines.push(format!(
                "{} {}",
                g_mem,
                gauge("Mem ", sys.used_memory(), sys.total_memory(), style, width)
            ));
        }
        if enabled("swap") {
            lines.push(format!(
                "{} {}",
                g_swap,
                gauge("Swap", sys.used_swap(), sys.total_swap(), style, width)
            ));
        }
    }

    if enabled("disk") {
        lines.extend(disk_lines(
            style,
            width,
            args.include_tmpfs.unwrap_or(false),
            g_disk,
        ));
    }

    if enabled("battery") {
        lines.extend(battery_lines(style, width, g_batt));
    }

    if lines.is_empty() {
        lines.push("Gauges: nothing to measure".to_string());
    }
    lines
}

/// `[####......] used / total (NN%)` for any used/total pair.
fn gauge(label: &str, used: u64, total: u64, style: &str, width: usize) -> String {
    let percent = percent(used, total);
    format!(
        "{} {} {} / {} ({:.0}%)",
        label,
        bar(percent, width, style),
        human(used),
        human(total),
        percent
    )
}

fn percent(used: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        used as f64 * 100.0 / total as f64
    }
}

/// Renders a `[////___]` / `[###....]` bar. `auto` uses slashes under 50 %
/// and hashes from 50 % up; the explicit styles always keep one look.
fn bar(percent: f64, width: usize, style: &str) -> String {
    let width = width.clamp(3, 40);
    let clamped = percent.clamp(0.0, 100.0);
    let filled = ((clamped / 100.0) * width as f64).round() as usize;
    let (fill, empty) = match style {
        "slashes" => ('/', '_'),
        "blocks" => ('█', '░'),
        "hash" => ('#', '.'),
        _ => {
            if clamped < 50.0 {
                ('/', '_')
            } else {
                ('#', '.')
            }
        }
    };
    format!(
        "[{}{}]",
        fill.to_string().repeat(filled),
        empty.to_string().repeat(width - filled)
    )
}

fn human(bytes: u64) -> String {
    const UNITS: [(&str, u64); 5] = [
        ("TiB", 1 << 40),
        ("GiB", 1 << 30),
        ("MiB", 1 << 20),
        ("KiB", 1 << 10),
        ("B", 1),
    ];
    for (unit, factor) in UNITS {
        if bytes >= factor {
            return format!("{:.2} {}", bytes as f64 / factor as f64, unit);
        }
    }
    "0 B".to_string()
}

fn disk_lines(style: &str, width: usize, include_tmpfs: bool, glyph: &str) -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut seen = std::collections::HashSet::new();
    let mut rows: Vec<(String, String)> = Vec::new();

    for disk in disks.list() {
        let total = disk.total_space();
        if total == 0 {
            continue;
        }
        let fs = disk.file_system().to_string_lossy().to_string();
        if !include_tmpfs && fs == "tmpfs" {
            continue;
        }
        if PSEUDO_FS.contains(&fs.as_str()) {
            continue;
        }
        let mount = disk.mount_point().display().to_string();
        if mount.starts_with("/run")
            || mount.starts_with("/sys")
            || mount.starts_with("/proc")
            || mount.starts_with("/dev")
        {
            continue;
        }
        let used = total - disk.available_space();
        if !seen.insert((fs.clone(), total, used)) {
            continue;
        }
        let row = gauge("Disk", used, total, style, width);
        let line = format!("{} {} {} {}", glyph, row, fs, mount);
        rows.push((mount, line));
    }

    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows.into_iter().map(|(_, line)| line).collect()
}

#[cfg(target_os = "linux")]
fn battery_lines(style: &str, width: usize, glyph: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") else {
        return lines;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let kind = std::fs::read_to_string(path.join("type")).unwrap_or_default();
        if kind.trim() != "Battery" {
            continue;
        }
        let Ok(capacity) = std::fs::read_to_string(path.join("capacity")) else {
            continue;
        };
        let Ok(capacity) = capacity.trim().parse::<u64>() else {
            continue;
        };
        let status = std::fs::read_to_string(path.join("status"))
            .map(|value| value.trim().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());
        let name = path
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| "Battery".to_string());
        lines.push(format!(
            "{} Batt {} {}% [{}] {}",
            glyph,
            bar(capacity as f64, width, style),
            capacity,
            status,
            name
        ));
    }
    lines
}

#[cfg(not(target_os = "linux"))]
fn battery_lines(_style: &str, _width: usize, _glyph: &str) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_switches_style_by_fill() {
        assert_eq!(bar(25.0, 8, "auto"), "[//______]");
        assert_eq!(bar(75.0, 8, "auto"), "[######..]");
    }

    #[test]
    fn explicit_styles_are_stable() {
        assert_eq!(bar(50.0, 4, "slashes"), "[//__]");
        assert_eq!(bar(50.0, 4, "hash"), "[##..]");
        assert_eq!(bar(50.0, 4, "blocks"), "[██░░]");
    }

    #[test]
    fn bounds_are_clamped() {
        assert_eq!(bar(0.0, 6, "hash"), "[......]");
        assert_eq!(bar(100.0, 6, "hash"), "[######]");
        assert_eq!(bar(200.0, 6, "hash"), "[######]");
    }

    #[test]
    fn glyph_sets_switch() {
        assert_eq!(glyphs("cjk"), ("存", "换", "盘", "电"));
        assert_eq!(glyphs("jp"), ("存", "換", "盤", "電"));
        assert_eq!(glyphs("nerd").0, GLYPH_MEM);
        assert_eq!(
            glyphs("unknown"),
            (GLYPH_MEM, GLYPH_SWAP, GLYPH_DISK, GLYPH_BATT)
        );
    }

    #[test]
    fn human_formats_units() {
        assert_eq!(human(0), "0 B");
        assert_eq!(human(1024), "1.00 KiB");
        assert_eq!(human(3 * (1 << 30)), "3.00 GiB");
    }

    #[test]
    fn gauge_formats_percent() {
        assert_eq!(
            gauge("Mem ", 1 << 30, 4 << 30, "hash", 8),
            "Mem  [##......] 1.00 GiB / 4.00 GiB (25%)"
        );
    }
}
