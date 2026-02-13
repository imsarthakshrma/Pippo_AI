use sqlx::FromRow;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Represents a single trade record in the database.
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Trade {
    /// Unique identifier for the trade (UUID).
    pub id: String,
    /// The market being traded on.
    pub market_id: String,
    /// The predicted outcome (e.g., "Yes", "No").
    pub outcome: String,
    /// Amount of capital allocated to this trade in USD.
    pub amount_usd: f64,
    /// The market price (implied probability) at the time of entry.
    pub price: f64,
    /// Current status of the trade (e.g., "Open", "Closed").
    pub status: String,
    /// Timestamp of trade creation.
    pub created_at: DateTime<Utc>,
}

/// Captures the state of a market at a specific point in time for analysis and backtesting.
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MarketSnapshot {
    /// Unique identifier for the snapshot.
    pub id: String,
    /// The market being captured.
    pub market_id: String,
    /// Current implied probability/price of the outcome.
    pub odds: f64,
    /// Trading volume over the last 24 hours.
    pub volume_24h: f64,
    /// Timestamp of when the scanner captured this state.
    pub scanned_at: DateTime<Utc>,
}

/// Stores the internal reasoning process (thinking blocks) of an agent.
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ThinkingLog {
    /// Unique identifier for the log entry.
    pub id: String,
    /// Optional link to a specific trade if this thinking led to an execution.
    pub trade_id: Option<String>,
    /// The raw markdown/text of Claude's internal reasoning.
    pub raw_thinking: String,
    /// Number of tokens in the input prompt.
    pub prompt_tokens: i32,
    /// Number of tokens used for internal reasoning.
    pub thinking_tokens: i32,
    /// Timestamp of when the log was generated.
    pub created_at: DateTime<Utc>,
}

/// Records capital redistribution between agents within the colony.
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CapitalTransfer {
    /// Unique identifier for the transfer.
    pub id: String,
    /// Agent ID sending the capital.
    pub from_agent: String,
    /// Agent ID receiving the capital.
    pub to_agent: String,
    /// Amount transferred in cents (to prevent floating-point errors).
    pub amount: i64,
    /// The reason for the redistribution (e.g., "Performance Reward", "Risk Rebalancing").
    pub reason: String,
    /// Timestamp of the transfer.
    pub timestamp: DateTime<Utc>,
}
