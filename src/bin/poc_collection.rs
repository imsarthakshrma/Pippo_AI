use anyhow::Result;
use chrono::{Utc, Duration};
use pippo::backtesting::data_collector::HistoricalDataCollector;
use pippo::backtesting::data_sources::polymarket_scraper::HistoricalMarket;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let collector = HistoricalDataCollector::new();
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(400);
    
    println!("Starting proof-of-concept data collection for 1 week...");
    let markets: Vec<HistoricalMarket> = collector.collect_full_dataset(start_date, end_date).await?;
    
    println!("Successfully collected {} markets.", markets.len());
    Ok(())
}
