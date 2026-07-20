use std::process::Command;
use xfetch_plugin_api::{EmptyArgs, read_info_plugin_args_or_default, write_info_lines};

fn main() {
    let _args = match read_info_plugin_args_or_default::<EmptyArgs>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let lines = get_docker_info();

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn get_docker_info() -> Vec<String> {
    let mut result = Vec::new();

    let info_output = Command::new("docker")
        .args(["info", "--format", "{{.Containers}} {{.ContainersRunning}} {{.ContainersPaused}} {{.ContainersStopped}}"])
        .output();

    match info_output {
        Ok(output) if output.status.success() => {
            let stats = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stats.split_whitespace().collect();
            if parts.len() >= 4 {
                let total = parts[0];
                let running = parts[1];
                let paused = parts[2];
                let stopped = parts[3];
                result.push(format!(" Containers: {} total", total));
                result.push(format!("  ▶ {} running", running));
                result.push(format!("  ⏸ {} paused", paused));
                result.push(format!("  ⏹ {} stopped", stopped));
            }
        }
        Ok(_) => result.push(" Docker: daemon not running".to_string()),
        Err(_) => result.push(" Docker: not found".to_string()),
    }

    result
}
