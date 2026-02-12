use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricalMarket {
    pub id: String,
    pub question: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub outcome: Option<String>,
    pub odds_history: Vec<OddsPoint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OddsPoint {
    pub timestamp: DateTime<Utc>,
    pub odds: Vec<f64>,
}

pub struct DataCollector;

impl DataCollector {
    pub async fn fetch_historical_polymarket(range_months: u32) -> Result<Vec<HistoricalMarket>> {
        // TODO: Ingest historical data from Polymarket API or local DB
        info!("Ingesting {} months of historical Polymarket data...", range_months);
        Ok(vec![])
    }

    pub async fn fetch_historical_weather(zip: &str, start: DateTime<Utc>) -> Result<()> {
        // TODO: Ingest NOAA historical data
        info!("Ingesting historical weather data for {} since {}", zip, start);
        Ok(())
    }
}
