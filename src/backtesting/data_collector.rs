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
    pub fn new(noaa_token: String) -> Self {
        Self {
            polymarket: PolymarketHistoricalScraper::new(),
            noaa: NOAAHistoricalScraper::new(noaa_token),
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
                    let location = extract_location(&market.question).unwrap_or("NYC".to_string());
                    match self.noaa.fetch_weather_for_date(&location, ed.date_naive()).await {
                        Ok(data) => info!("Enriched weather for {}: {:?}", location, data),
                        Err(e) => error!("Failed to fetch weather for {}: {:?}", location, e),
                    }
                }
                MarketType::Sports => {
                    let sport = extract_sport(&market.question).unwrap_or("NBA".to_string());
                    match self.sports.fetch_game_results(&sport, ed.date_naive()).await {
                        Ok(data) => info!("Enriched sports for {}: {:?}", sport, data),
                        Err(e) => error!("Failed to fetch sports for {}: {:?}", sport, e),
                    }
                }
                MarketType::Crypto => {
                    let asset = extract_asset(&market.question).unwrap_or("BTC".to_string());
                    let start = ed - chrono::Duration::hours(24);
                    match self.crypto.fetch_price_history(&asset, start, ed).await {
                        Ok(data) => info!("Enriched crypto for {}: {:?}", asset, data),
                        Err(e) => error!("Failed to fetch crypto for {}: {:?}", asset, e),
                    }
                }
                _ => {
                    let topic = extract_topic(&market.question).unwrap_or("all".to_string());
                    match self.sentiment.fetch_reddit_sentiment(&topic, ed.date_naive()).await {
                        Ok(data) => info!("Enriched sentiment for {}: {:?}", topic, data),
                        Err(e) => error!("Failed to fetch sentiment for {}: {:?}", topic, e),
                    }
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
        } else if is_sports_market(&q_lower) {
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

/// Helper to determine if a market is sports-related using specific signals.
fn is_sports_market(q: &str) -> bool {
    let signals = ["nba", "nfl", "mlb", "nhl", "goal", "touchdown", "home run", "score", "mvp"];
    let mut count = 0;
    for &s in &signals {
        // Simple word-boundary check
        let pattern = format!(" {} ", s);
        if q.contains(&pattern) || q.starts_with(s) || q.ends_with(s) {
            count += 1;
            if s == "nba" || s == "nfl" || s == "mlb" || s == "nhl" {
                return true; // Direct league hit
            }
        }
    }
    count >= 2
}

fn extract_location(q: &str) -> Option<String> {
    let q_lower = q.to_lowercase();
    if q_lower.contains("nyc") || q_lower.contains("new york") { Some("NYC".to_string()) }
    else if q_lower.contains("london") { Some("London".to_string()) }
    else if q_lower.contains("tokyo") { Some("Tokyo".to_string()) }
    else { None }
}

fn extract_sport(q: &str) -> Option<String> {
    let q_lower = q.to_lowercase();
    if q_lower.contains("nba") { Some("NBA".to_string()) }
    else if q_lower.contains("nfl") { Some("NFL".to_string()) }
    else if q_lower.contains("mlb") { Some("MLB".to_string()) }
    else if q_lower.contains("nhl") { Some("NHL".to_string()) }
    else { None }
}

fn extract_asset(q: &str) -> Option<String> {
    let q_lower = q.to_lowercase();
    if q_lower.contains("btc") || q_lower.contains("bitcoin") { Some("BTC".to_string()) }
    else if q_lower.contains("eth") || q_lower.contains("ethereum") { Some("ETH".to_string()) }
    else { None }
}

fn extract_topic(q: &str) -> Option<String> {
    let q_lower = q.to_lowercase();
    if q_lower.contains("election") || q_lower.contains("president") { Some("politics".to_string()) }
    else { None }
}
