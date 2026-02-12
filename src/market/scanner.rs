use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Market {
    pub id: String,
    pub question: String,
    pub outcome_assets: Vec<String>,
    pub current_odds: Vec<f64>,
}

pub struct MarketScanner;

impl MarketScanner {
    pub async fn scan_active_markets() -> Result<Vec<Market>> {
        // Mock data for initial testing
        Ok(vec![
            Market {
                id: "market_1".to_string(),
                question: "Will Bitcoin hit $100k by March 2026?".to_string(),
                outcome_assets: vec!["Yes".to_string(), "No".to_string()],
                current_odds: vec![0.45, 0.55],
            },
            Market {
                id: "market_2".to_string(),
                question: "Will it rain in NYC on Feb 15?".to_string(),
                outcome_assets: vec!["Yes".to_string(), "No".to_string()],
                current_odds: vec![0.30, 0.70],
            },
        ])
    }
}
