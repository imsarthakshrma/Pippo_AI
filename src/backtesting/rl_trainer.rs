use crate::db::DbManager;
use anyhow::Result;
use tracing::info;

pub struct RlTrainer {
    db: DbManager,
}

impl RlTrainer {
    pub fn new(db: DbManager) -> Self {
        Self { db }
    }

    pub async fn train_on_history(&self) -> Result<()> {
        info!("Starting RL training on historical data...");
        // TODO: Implement offline training logic
        Ok(())
    }
}
