use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

pub struct CryptoHistoricalScraper {
    client: Client,
}

impl CryptoHistoricalScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_price_history(
        &self,
        _coin: &str,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<Vec<PricePoint>> {
        // Placeholder for real crypto API call (e.g. CoinGecko)
        Ok(vec![])
    }
}

#[derive(Debug, Deserialize)]
pub struct PricePoint {
    pub timestamp: i64,
    pub price: f64,
}
