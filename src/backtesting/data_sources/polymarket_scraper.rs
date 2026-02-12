use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

pub struct PolymarketHistoricalScraper {
    client: Client,
}

use tracing::info;

impl PolymarketHistoricalScraper {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }

    pub async fn fetch_historical_markets(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<HistoricalMarket>> {
        let url = "https://gamma-api.polymarket.com/markets";
        let mut all_markets = Vec::new();
        let mut offset = 0;
        
        loop {
            info!("Fetching page at offset {}, total collected: {}", offset, all_markets.len());
            let response = self.client
                .get(url)
                .query(&[
                    ("closed", "true"),
                    ("limit", "100"),
                    ("offset", &offset.to_string()),
                ])
                .send()
                .await?
                .json::<Vec<HistoricalMarket>>()
                .await?;
            
            if response.is_empty() {
                info!("Empty response at offset {}", offset);
                break;
            }
            
            if offset == 0 {
                if let Some(first) = response.first() {
                    info!("First market sample: ID={}, EndDate={:?}, Question={}", first.id, first.end_date, first.question);
                } else {
                    info!("First page response is empty!");
                }
            }
            
            info!("Fetched {} markets from page", response.len());
            
            let filtered: Vec<HistoricalMarket> = response.into_iter()
                .filter(|m| {
                    if let Some(ed) = m.end_date {
                        ed >= start_date && ed <= end_date
                    } else {
                        false
                    }
                })
                .collect();
            
            all_markets.extend(filtered);
            offset += 100;
            
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            if offset > 5000 { break; } 
        }
        
        Ok(all_markets)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HistoricalMarket {
    pub id: String,
    pub question: String,
    pub description: Option<String>,
    #[serde(alias = "endDate")]
    pub end_date: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
    #[serde(deserialize_with = "deserialize_nullable_f64_from_str", default)]
    pub volume: Option<f64>,
}

fn deserialize_nullable_f64_from_str<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s.parse::<f64>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
