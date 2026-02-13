use serde::Deserialize;
use anyhow::Result;
use config::{Config, File};

#[derive(Debug, Deserialize, Clone)]
pub struct PippoConfig {
    pub claude: ClaudeConfig,
    pub polymarket: PolymarketConfig,
    pub trading: TradingConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClaudeConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: usize,
    pub thinking_budget_default: usize,
    pub thinking_budget_complex: usize,
    /// Temperature for personality chat (0.0–1.0). Higher = more expressive.
    #[serde(default = "default_chat_temperature")]
    pub chat_temperature: f64,
}

fn default_chat_temperature() -> f64 { 0.75 }

#[derive(Debug, Deserialize, Clone)]
pub struct PolymarketConfig {
    pub api_key: String,
    pub secret: String,
    pub passphrase: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradingConfig {
    pub max_kelly_fraction: f64,
    pub min_edge: f64,
    pub balance_usd: f64,
    pub interval_minutes: u64,
    pub database_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub log_level: String,
    pub save_thinking_logs: bool,
}

pub fn load() -> Result<PippoConfig> {
    let s = Config::builder()
        .add_source(File::with_name("config"))
        .add_source(config::Environment::with_prefix("PIPPO"))
        .build()?;

    Ok(s.try_deserialize()?)
}
