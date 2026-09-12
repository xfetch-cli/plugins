//! `wasm-crypto`: spot prices for a few assets, fetched over the sandboxed
//! HTTP host call.
//!
//! The manifest allowlists exactly one origin (`api.coinbase.com`); any other
//! URL is denied by the runtime. The plugin works natively too, where host
//! calls report `Unsupported` and the fallback lines are emitted instead.

use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;
use xfetch_guest_api::{HostCallError, http_request, log};
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

/// One HTTP request per asset; a generous budget covers slow connections.
const BUDGET: Duration = Duration::from_secs(12);

/// Per-request timeout passed to the host.
const REQUEST_TIMEOUT_MS: u64 = 4_000;

/// Arguments accepted under the plugin's `args` config object.
#[derive(Debug, Default, Deserialize)]
struct CryptoArgs {
    /// Asset symbols to quote (default: BTC, ETH, SOL).
    assets: Option<Vec<String>>,
    /// Quote currency (default: USD).
    currency: Option<String>,
}

fn main() {
    let lines = match with_timeout(BUDGET, build_lines) {
        Ok(Ok(lines)) => lines,
        Ok(Err(err)) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("wasm-crypto: timed out");
            std::process::exit(1);
        }
    };

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

/// Quotes every configured asset, degrading to one fallback line per failure.
fn build_lines() -> Result<Vec<String>, String> {
    let args = read_info_plugin_args_or_default::<CryptoArgs>().map_err(|err| err.to_string())?;

    let assets = args
        .assets
        .filter(|assets| !assets.is_empty())
        .unwrap_or_else(|| vec!["BTC".to_string(), "ETH".to_string(), "SOL".to_string()]);
    let currency = args
        .currency
        .map(|currency| currency.to_ascii_uppercase())
        .unwrap_or_else(|| "USD".to_string());

    let mut lines = Vec::with_capacity(assets.len());
    for asset in assets.iter().take(5) {
        let asset = asset.to_ascii_uppercase();
        lines.push(quote(&asset, &currency));
    }
    Ok(lines)
}

/// Fetches one spot price and formats it, or explains why it failed.
fn quote(asset: &str, currency: &str) -> String {
    let url = format!(
        "https://api.coinbase.com/v2/prices/{}-{}/spot",
        asset, currency
    );

    log("info", &format!("wasm-crypto: fetching {}", url));
    let response = match http_request("GET", &url, &[], None, Some(REQUEST_TIMEOUT_MS)) {
        Ok(response) => response,
        Err(err) => return format!("{}: unavailable ({})", asset, describe(&err)),
    };

    if response.status != 200 {
        return format!("{}: HTTP {}", asset, response.status);
    }

    let body: Value = match serde_json::from_slice(&response.body) {
        Ok(body) => body,
        Err(_) => return format!("{}: malformed response", asset),
    };

    let amount = body
        .get("data")
        .and_then(|data| data.get("amount"))
        .and_then(Value::as_str)
        .and_then(|amount| amount.parse::<f64>().ok());

    match amount {
        Some(amount) => format!("{}: {}{}", asset, symbol(currency), format_amount(amount)),
        None => format!("{}: missing price", asset),
    }
}

/// One-character currency symbol when known, otherwise the code.
fn symbol(currency: &str) -> String {
    match currency {
        "USD" => "$".to_string(),
        "EUR" => "\u{20ac}".to_string(),
        "GBP" => "\u{a3}".to_string(),
        other => format!("{} ", other),
    }
}

/// Formats with two decimals and thousands separators.
fn format_amount(amount: f64) -> String {
    let raw = format!("{:.2}", amount);
    let (integer, decimals) = raw.split_once('.').unwrap_or((raw.as_str(), "00"));

    let mut grouped = String::new();
    for (index, ch) in integer.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(ch);
    }
    let grouped: String = grouped.chars().rev().collect();
    format!("{}.{}", grouped, decimals)
}

/// Short, user-facing failure reason.
fn describe(err: &HostCallError) -> String {
    format!("{}", err.kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_amounts_with_grouping() {
        assert_eq!(format_amount(63450.1), "63,450.10");
        assert_eq!(format_amount(9.5), "9.50");
        assert_eq!(format_amount(1234567.891), "1,234,567.89");
    }

    #[test]
    fn maps_known_currency_symbols() {
        assert_eq!(symbol("USD"), "$");
        assert_eq!(symbol("EUR"), "\u{20ac}");
        assert_eq!(symbol("JPY"), "JPY ");
    }
}
