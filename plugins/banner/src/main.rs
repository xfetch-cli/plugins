//! Puts a short Chinese/Japanese proverb banner under (or above) the ASCII
//! logo without touching the logo file: the core hands us the logo lines and
//! we return them with the banner appended.
//!
//! `style` in `logo_animation` selects the placement and the proverb:
//! `banner` (random, below), `banner-above`, `banner-static` (first proverb)
//! or `banner-N` (1-based index into the list). When `frames_path` is set, its
//! first frame is used as the banner text instead of the built-in list.

use std::time::Duration;
use xfetch_plugin_api::{
    AnimationFrame, LogoAnimationRequest, read_logo_animation_request, with_timeout,
    write_logo_animation_frames,
};

const BUDGET: Duration = Duration::from_secs(3);

const PROVERBS: &[&str] = &[
    "千里之行，始于足下",
    "知足者常乐",
    "温故而知新",
    "水滴石穿",
    "有志者事竟成",
    "失败乃成功之母",
    "七転び八起き",
    "継続は力なり",
    "一期一会",
    "初心忘るべからず",
    "案ずるより産むが易し",
    "石の上にも三年",
    "花鳥風月",
    "明日は明日の風が吹く",
];

fn main() {
    let frames = with_timeout(BUDGET, || {
        let request = match read_logo_animation_request() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        vec![build_frame(&request)]
    })
    .unwrap_or_else(|_| {
        vec![AnimationFrame::new(
            0,
            vec!["banner: timed out".to_string()],
        )]
    });

    if let Err(err) = write_logo_animation_frames(frames) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn build_frame(request: &LogoAnimationRequest) -> AnimationFrame {
    let style = request.args.style.as_deref().unwrap_or("banner");
    let above = style.contains("above");
    let text = banner_text(request, style);
    let width = logo_width(&request.lines);
    let banner: Vec<String> = text.iter().map(|line| center(line, width)).collect();

    let mut lines = Vec::new();
    if above {
        lines.extend(banner);
        lines.push(String::new());
        lines.extend(request.lines.clone());
    } else {
        lines.extend(request.lines.clone());
        lines.push(String::new());
        lines.extend(banner);
    }
    AnimationFrame::new(0, lines)
}

/// Banner lines: the first `frames_path` frame when present, else a proverb.
fn banner_text(request: &LogoAnimationRequest, style: &str) -> Vec<String> {
    if let Some(sets) = request.frames.as_ref()
        && let Some(first) = sets.iter().find(|set| !set.is_empty())
    {
        return first.clone();
    }

    let index = if let Some(rest) = style.strip_prefix("banner-") {
        match rest {
            "above" | "below" | "static" | "" => 0,
            digits => digits.parse::<usize>().unwrap_or(1).saturating_sub(1) % PROVERBS.len(),
        }
    } else {
        now_nanos() as usize % PROVERBS.len()
    };
    vec![PROVERBS[index].to_string()]
}

fn now_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u128)
        .unwrap_or(0)
}

/// Visible width of the logo (widest line), used to centre the banner.
fn logo_width(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| display_width(line))
        .max()
        .unwrap_or(0)
}

fn center(text: &str, width: usize) -> String {
    let text_width = display_width(text);
    if text_width >= width {
        return text.to_string();
    }
    let left = (width - text_width) / 2;
    let right = width - text_width - left;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

/// Display width: East Asian wide characters count as two columns.
fn display_width(text: &str) -> usize {
    text.chars().map(|ch| if is_wide(ch) { 2 } else { 1 }).sum()
}

fn is_wide(ch: char) -> bool {
    let c = ch as u32;
    (0x1100..=0x115F).contains(&c)
        || (0x2E80..=0x303E).contains(&c)
        || (0x3041..=0x33FF).contains(&c)
        || (0x3400..=0x4DBF).contains(&c)
        || (0x4E00..=0x9FFF).contains(&c)
        || (0xA000..=0xA4CF).contains(&c)
        || (0xAC00..=0xD7A3).contains(&c)
        || (0xF900..=0xFAFF).contains(&c)
        || (0xFE30..=0xFE4F).contains(&c)
        || (0xFF00..=0xFF60).contains(&c)
        || (0xFFE0..=0xFFE6).contains(&c)
        || (0x1F300..=0x1F64F).contains(&c)
        || (0x1F900..=0x1F9FF).contains(&c)
        || (0x20000..=0x3FFFD).contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use xfetch_plugin_api::LogoAnimationArgs;

    fn request(style: &str) -> LogoAnimationRequest {
        LogoAnimationRequest::new(
            vec!["  ██  ".to_string(), " ████ ".to_string()],
            None,
            LogoAnimationArgs {
                style: Some(style.to_string()),
                ..LogoAnimationArgs::default()
            },
        )
    }

    #[test]
    fn banner_goes_below_the_logo() {
        let frame = build_frame(&request("banner-static"));
        assert_eq!(frame.lines.len(), 4);
        assert_eq!(frame.lines[0], "  ██  ");
        assert!(frame.lines[3].contains(PROVERBS[0]));
    }

    #[test]
    fn banner_above_puts_it_first() {
        let frame = build_frame(&request("banner-above"));
        assert!(frame.lines[0].contains(PROVERBS[0]));
        assert_eq!(frame.lines[1], "");
        assert_eq!(frame.lines[2], "  ██  ");
    }

    #[test]
    fn centered_line_matches_logo_width() {
        let req = LogoAnimationRequest::new(
            vec!["  ██  ".to_string(), " ████ ".to_string()],
            Some(vec![vec!["X".to_string()]]),
            LogoAnimationArgs {
                style: Some("banner".to_string()),
                ..LogoAnimationArgs::default()
            },
        );
        let frame = build_frame(&req);
        let banner = frame.lines.last().expect("banner line");
        assert_eq!(display_width(banner), display_width("  ██  "));
    }

    #[test]
    fn frames_path_overrides_the_list() {
        let req = LogoAnimationRequest::new(
            vec!["X".to_string()],
            Some(vec![vec!["我的信息".to_string()]]),
            LogoAnimationArgs {
                style: Some("banner".to_string()),
                ..LogoAnimationArgs::default()
            },
        );
        assert_eq!(banner_text(&req, "banner"), vec!["我的信息".to_string()]);
    }

    #[test]
    fn wide_characters_count_as_two() {
        assert_eq!(display_width("ab"), 2);
        assert_eq!(display_width("盘"), 2);
        assert_eq!(display_width("a盘"), 3);
    }
}
