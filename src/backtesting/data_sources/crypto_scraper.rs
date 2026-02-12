use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

/// A scraper for fetching historical cryptocurrency price data (e.g., from CoinGecko).
pub struct CryptoHistoricalScraper {
    client: Client,
}

impl CryptoHistoricalScraper {
    /// Creates a new `CryptoHistoricalScraper`.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetches price history for a specific coin within a date range.
    /// 
    /// currently this is a stub that returns an empty vector.
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

/// Represents a single price measurement at a specific time.
#[derive(Debug, Deserialize)]
pub struct PricePoint {
    /// Unix timestamp of the measurement.
    pub timestamp: i64,
    /// Price of the cryptocurrency in USD.
    pub price: f64,
}
