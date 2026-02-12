use anyhow::Result;
use chrono::{DateTime, Utc};
pub use crate::backtesting::data_sources::polymarket_scraper::{PolymarketHistoricalScraper, HistoricalMarket};
use crate::backtesting::data_sources::noaa_scraper::NOAAHistoricalScraper;
use crate::backtesting::data_sources::sports_scraper::SportsHistoricalScraper;
use crate::backtesting::data_sources::crypto_scraper::CryptoHistoricalScraper;
use crate::backtesting::data_sources::sentiment_scraper::SentimentHistoricalScraper;
use tracing::{info, error};

/// Orchestrates the collection of historical data from multiple sources.
/// 
/// It integrates Polymarket data with contextual information like weather, 
/// sports results, and social sentiment to build a comprehensive dataset 
/// for backtesting.
pub struct HistoricalDataCollector {
    polymarket: PolymarketHistoricalScraper,
    noaa: NOAAHistoricalScraper,
    sports: SportsHistoricalScraper,
    crypto: CryptoHistoricalScraper,
    sentiment: SentimentHistoricalScraper,
}

impl HistoricalDataCollector {
    /// Creates a new `HistoricalDataCollector` with initialized scrapers.
    pub fn new() -> Self {
        Self {
            polymarket: PolymarketHistoricalScraper::new(),
            noaa: NOAAHistoricalScraper::new("".to_string()),
            sports: SportsHistoricalScraper::new(),
            crypto: CryptoHistoricalScraper::new(),
            sentiment: SentimentHistoricalScraper::new(),
        }
    }

    /// Fetches and classifies a full dataset of historical markets within a date range.
    ///
    /// This method first retrieves resolved markets from Polymarket and then 
    /// attempts to enrich each market with relevant contextual data (weather, sports, etc.) 
    /// based on the market's category.
    pub async fn collect_full_dataset(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<HistoricalMarket>> {
        info!("Starting historical data collection from {} to {}", start_date, end_date);
        
        let markets: Vec<HistoricalMarket> = self.polymarket
            .fetch_historical_markets(start_date, end_date)
            .await?;
        
        info!("Found {} historical markets", markets.len());
        
        for market in &markets {
            info!("Processing market: {}", market.question);
            let market_type = self.classify_market(&market.question);
            
            let ed = match market.end_date {
                Some(date) => date,
                None => {
                    info!("Skipping market {} due to missing end_date", market.id);
                    continue;
                }
            };
            
            match market_type {
                MarketType::Weather => {
                    let _ = self.noaa.fetch_weather_for_date("NYC", ed.date_naive()).await;
                }
                MarketType::Sports => {
                    let _ = self.sports.fetch_game_results("NBA", ed.date_naive()).await;
                }
                MarketType::Crypto => {
                    let _ = self.crypto.fetch_price_history("BTC", ed - chrono::Duration::hours(24), ed).await;
                }
                _ => {
                    let _ = self.sentiment.fetch_reddit_sentiment("all", ed.date_naive()).await;
                }
            }
        }
        
        Ok(markets)
    }

    /// Categorizes a market question into a specific `MarketType` for targeted data enrichment.
    fn classify_market(&self, question: &str) -> MarketType {
        let q_lower = question.to_lowercase();
        if q_lower.contains("rain") || q_lower.contains("snow") || q_lower.contains("temperature") {
            MarketType::Weather
        } else if q_lower.contains("nba") || q_lower.contains("nfl") || q_lower.contains("win") {
            MarketType::Sports
        } else if q_lower.contains("bitcoin") || q_lower.contains("ethereum") || q_lower.contains("btc") {
            MarketType::Crypto
        } else if q_lower.contains("election") || q_lower.contains("president") {
            MarketType::Politics
        } else {
            MarketType::Other
        }
    }
}

/// Represents the broad category of a prediction market.
pub enum MarketType {
    /// Weather-related events (e.g., rainfall, temperature).
    Weather,
    /// Sporting events (e.g., NBA, NFL game results).
    Sports,
    /// Cryptocurrency price movements or technical milestones.
    Crypto,
    /// Political events (e.g., election outcomes).
    Politics,
    /// Any other market type not specifically categorized.
    Other,
}
