use std::process::Command;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, serde::Deserialize)]
struct PluginArgs {
    location: Option<String>,
    format: Option<String>,
}

fn main() {
    let args = match read_info_plugin_args_or_default::<PluginArgs>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let lines = get_weather(args.location.as_deref(), args.format.as_deref());

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_weather(location: Option<&str>, format: Option<&str>) -> Vec<String> {
    let loc = location.unwrap_or("");
    let fmt = format.unwrap_or("%C|%t|%w|%h|%p");

    let url = if loc.is_empty() {
        format!("https://wttr.in/?format={}", fmt)
    } else {
        format!("https://wttr.in/{}?format={}", loc, fmt)
    };

    let output = match Command::new("curl")
        .args(["-s", "--max-time", "10", &url])
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return vec![" Weather: could not fetch".to_string()],
    };

    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if raw.is_empty() {
        return vec![" Weather: no data".to_string()];
    }

    let parts: Vec<&str> = raw.split('|').map(|s| s.trim()).collect();
    let mut result = Vec::new();

    if parts.len() >= 4 {
        let condition = parts[0].trim();
        let temp = parts[1].trim();
        let wind = parts[2].trim();
        let humidity = parts[3].trim();

        if condition.is_empty() && temp.is_empty() && wind.is_empty() {
            result.push(format!(" Weather: {}", raw));
            return result;
        }

        let icon = match condition {
            c if c.contains("Clear") || c.contains("Sunny") => "\u{e30d}",
            c if c.contains("Cloud") || c.contains("Overcast") => "\u{e302}",
            c if c.contains("Rain") || c.contains("Drizzle") => "\u{e315}",
            c if c.contains("Snow") || c.contains("Ice") => "\u{e318}",
            c if c.contains("Thunder") || c.contains("Storm") => "\u{e31a}",
            c if c.contains("Fog") || c.contains("Mist") || c.contains("Haze") => "\u{e376}",
            c if c.contains("Partly") => "\u{e312}",
            _ => "\u{e30d}",
        };

        result.push(format!("{} {} {}", icon, temp, condition));

        let precip = parts.get(4).map(|s| s.trim()).unwrap_or("");
        result.push(format!("  \u{e36e} Humidity: {}", humidity));
        result.push(format!("  \u{e374} Wind: {}", wind));
        if !precip.is_empty() {
            result.push(format!("   Precipitation: {}", precip));
        }
    } else {
        result.push(format!(" Weather: {}", raw));
    }

    result
}
