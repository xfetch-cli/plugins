use xfetch_plugin_api::{
    AnimationFrame, read_logo_animation_request, write_logo_animation_frames,
};

fn main() {
    let request = match read_logo_animation_request() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let args = request.args;
    let fps = clamp(args.fps.unwrap_or(12), 1, 60);
    let frame_delay = 1000 / fps;
    let duration_ms = std::cmp::max(frame_delay, args.duration_ms.unwrap_or(1200));
    let frame_count = std::cmp::max(1, duration_ms / frame_delay);
    let style = args.style.as_deref().unwrap_or("sweep");
    let frame_sets = request.frames.unwrap_or_default();

    let frames: Vec<AnimationFrame> = match style {
        "frame" if !frame_sets.is_empty() => {
            generate_ascii_frame_animation(&frame_sets, frame_count, frame_delay)
        }
        "wave" => generate_wave_animation(&request.lines, frame_count, frame_delay),
        "rainbow" => generate_rainbow_animation(&request.lines, frame_count, frame_delay),
        "sparkle" => generate_sparkle_animation(&request.lines, frame_count, frame_delay),
        "breathing" => generate_breathing_animation(&request.lines, frame_count, frame_delay),
        "none" => generate_static_animation(&request.lines, frame_count, frame_delay),
        _ => generate_sweep_animation(&request.lines, frame_count, frame_delay),
    };

    if let Err(err) = write_logo_animation_frames(frames) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn generate_sweep_animation(lines: &[String], count: u64, delay: u64) -> Vec<AnimationFrame> {
    (0..count)
        .map(|i| AnimationFrame {
            delay_ms: delay,
            lines: lines.iter().map(|l| color_sweep(l, i as usize)).collect(),
        })
        .collect()
}

fn generate_wave_animation(lines: &[String], count: u64, delay: u64) -> Vec<AnimationFrame> {
    (0..count)
        .map(|i| AnimationFrame {
            delay_ms: delay,
            lines: lines.iter().map(|l| color_wave(l, i as usize)).collect(),
        })
        .collect()
}

fn generate_rainbow_animation(lines: &[String], count: u64, delay: u64) -> Vec<AnimationFrame> {
    (0..count)
        .map(|i| AnimationFrame {
            delay_ms: delay,
            lines: lines
                .iter()
                .map(|l| color_rainbow(l, i as usize, count as usize))
                .collect(),
        })
        .collect()
}

fn generate_sparkle_animation(lines: &[String], count: u64, delay: u64) -> Vec<AnimationFrame> {
    let mut rng: u32 = 1;
    (0..count)
        .map(|_| {
            rng = rng.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            AnimationFrame {
                delay_ms: delay,
                lines: lines.iter().map(|l| color_sparkle(l, rng)).collect(),
            }
        })
        .collect()
}

fn generate_breathing_animation(lines: &[String], count: u64, delay: u64) -> Vec<AnimationFrame> {
    (0..count)
        .map(|i| AnimationFrame {
            delay_ms: delay,
            lines: lines
                .iter()
                .map(|l| color_breathing(l, i as usize, count as usize))
                .collect(),
        })
        .collect()
}

fn generate_static_animation(lines: &[String], _count: u64, delay: u64) -> Vec<AnimationFrame> {
    vec![AnimationFrame {
        delay_ms: delay,
        lines: lines.to_vec(),
    }]
}

fn generate_ascii_frame_animation(
    frame_sets: &[Vec<String>],
    count: u64,
    delay: u64,
) -> Vec<AnimationFrame> {
    if frame_sets.is_empty() {
        return Vec::new();
    }
    let max_h = frame_sets.iter().map(|f| f.len()).max().unwrap_or(0);
    let normalized: Vec<Vec<String>> = frame_sets
        .iter()
        .map(|f| {
            let mut n = f.clone();
            while n.len() < max_h {
                n.push(String::new());
            }
            n
        })
        .collect();

    (0..count)
        .map(|i| {
            let idx = i as usize % normalized.len();
            AnimationFrame {
                delay_ms: delay,
                lines: normalized[idx].clone(),
            }
        })
        .collect()
}

fn color_sweep(line: &str, offset: usize) -> String {
    const PALETTE: &[u8] = &[31, 32, 33, 34, 35, 36];
    let mut out = String::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let color = PALETTE[(idx + offset) % PALETTE.len()];
        out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
    }
    out
}

fn color_wave(line: &str, frame: usize) -> String {
    const PALETTE: &[u8] = &[31, 32, 33, 34, 35, 36];
    let mut out = String::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let wave = ((idx as f64 * 0.5 + frame as f64 * 0.3).sin() * 3.0) as isize;
        let color = PALETTE[((wave + 3).clamp(0, 5) as usize) % PALETTE.len()];
        out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
    }
    out
}

fn color_rainbow(line: &str, frame: usize, total: usize) -> String {
    let palette = [
        (196u8, 0u8, 0u8),
        (208, 128, 0),
        (220, 220, 0),
        (0, 200, 0),
        (0, 150, 200),
        (100, 50, 200),
    ];
    let line_len = line.chars().count();
    let total = total.max(1);
    let mut out = String::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let t = (idx as f64 / line_len.max(1) as f64 + frame as f64 / total as f64) % 1.0;
        let pi = t * (palette.len() - 1) as f64;
        let i0 = pi.floor() as usize;
        let i1 = (i0 + 1).min(palette.len() - 1);
        let frac = pi - pi.floor();
        let (r1, g1, b1) = palette[i0];
        let (r2, g2, b2) = palette[i1];
        let r = (r1 as f64 * (1.0 - frac) + r2 as f64 * frac) as u8;
        let g = (g1 as f64 * (1.0 - frac) + g2 as f64 * frac) as u8;
        let b = (b1 as f64 * (1.0 - frac) + b2 as f64 * frac) as u8;
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch));
    }
    out
}

fn color_sparkle(line: &str, seed: u32) -> String {
    const PALETTE: &[u8] = &[31, 32, 33, 34, 35, 36, 91, 92, 93, 94, 95, 96];
    let mut rng = seed;
    let mut out = String::new();
    for ch in line.chars() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        rng = rng.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let bright = (rng >> 16) & 0xFF;
        if bright > 200 {
            let color = PALETTE[(rng as usize) % PALETTE.len()];
            out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
        } else {
            out.push(ch);
        }
    }
    out
}

fn color_breathing(line: &str, frame: usize, total: usize) -> String {
    let total = total.max(1);
    let phase = (frame as f64 / total as f64 * std::f64::consts::PI * 2.0).sin();
    let brightness = ((phase * 0.5 + 0.5) * 155.0 + 100.0) as u8;
    let mut out = String::new();
    for ch in line.chars() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        out.push_str(&format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            brightness,
            brightness / 2,
            brightness / 3,
            ch
        ));
    }
    out
}

fn clamp(value: u64, min: u64, max: u64) -> u64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
