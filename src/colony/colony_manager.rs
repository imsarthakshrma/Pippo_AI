use crate::colony::traits::TradingPersonality;
use crate::colony::variants::*;
use std::sync::Arc;
use anyhow::Result;
use tracing::info;

pub struct ColonyManager {
    agents: Vec<Arc<dyn TradingPersonality>>,
}

impl ColonyManager {
    pub fn new() -> Self {
        Self {
            agents: vec![
                Arc::new(PippoAlpha),
                Arc::new(PippoBeta),
                Arc::new(PippoGamma),
                Arc::new(PippoDelta),
                Arc::new(PippoOmega),
            ],
        }
    }

    pub fn agents(&self) -> &[Arc<dyn TradingPersonality>] {
        &self.agents
    }

    pub async fn submit_vote(&self, agent_id: &str, market_id: &str, vote: bool) -> Result<()> {
        info!("Agent {} voted {} on market {}", agent_id, if vote { "YES" } else { "NO" }, market_id);
        // TODO: Store vote in shared SQLite table
        Ok(())
    }

    pub async fn get_colony_trades(&self) -> Result<Vec<String>> {
        // TODO: Query shared trades table for visibility
        Ok(vec![])
    }
}
