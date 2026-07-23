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
    let fmt = format.unwrap_or("%c+%t+%w+%h+%p");

    let mut url = if loc.is_empty() {
        "https://wttr.in/?format={}".to_string()
    } else {
        format!("https://wttr.in/{}?format={{}}", loc)
    };

    url = url.replace("{}", fmt);

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

    let parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
    let mut result = Vec::new();

    if parts.len() >= 4 {
        let condition = parts[0];
        let temp = parts[1];
        let wind = parts[2];
        let humidity = parts[3];

        let icon = match condition {
            c if c.contains("Clear") || c.contains("Sunny") => "☀",
            c if c.contains("Cloud") || c.contains("Overcast") => "☁",
            c if c.contains("Rain") || c.contains("Drizzle") => "🌧",
            c if c.contains("Snow") => "🌨",
            c if c.contains("Thunder") || c.contains("Storm") => "⛈",
            c if c.contains("Fog") || c.contains("Mist") => "🌫",
            c if c.contains("Partly") => "⛅",
            _ => "",
        };

        result.push(format!("{} {} {}", icon, temp, condition));

        if parts.len() >= 5 {
            let precip = parts[4];
            result.push(format!("   Humidity: {}", humidity));
            result.push(format!("   Wind: {}", wind));
            result.push(format!("   Precipitation: {}", precip));
        } else {
            result.push(format!("   Humidity: {}", humidity));
            result.push(format!("   Wind: {}", wind));
        }
    } else {
        result.push(format!(" Weather: {}", raw));
    }

    result
}
