use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use anyhow::Result;

pub struct DbManager {
    pool: Pool<Sqlite>,
}

impl DbManager {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    pub async fn save_trade(&self, trade: &crate::db::schema::Trade) -> Result<()> {
        sqlx::query(
            "INSERT INTO trades (id, market_id, outcome, amount_usd, price, status) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&trade.id)
        .bind(&trade.market_id)
        .bind(&trade.outcome)
        .bind(trade.amount_usd)
        .bind(trade.price)
        .bind(&trade.status)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_thinking_log(&self, log: &crate::db::schema::ThinkingLog) -> Result<()> {
        sqlx::query(
            "INSERT INTO thinking_logs (id, trade_id, raw_thinking, prompt_tokens, thinking_tokens) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&log.id)
        .bind(&log.trade_id)
        .bind(&log.raw_thinking)
        .bind(log.prompt_tokens)
        .bind(log.thinking_tokens)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
