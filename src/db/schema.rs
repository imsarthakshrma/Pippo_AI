use sqlx::FromRow;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub market_id: String,
    pub outcome: String,
    pub amount_usd: f64,
    pub price: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub id: String,
    pub market_id: String,
    pub odds: f64,
    pub volume_24h: f64,
    pub scanned_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ThinkingLog {
    pub id: String,
    pub trade_id: Option<String>,
    pub raw_thinking: String,
    pub prompt_tokens: i32,
    pub thinking_tokens: i32,
    pub created_at: DateTime<Utc>,
}
