use crate::colony::traits::TradingPersonality;
use crate::colony::variants::*;
use std::sync::Arc;
use anyhow::Result;
use tracing::info;

/// Manages the collection of agents, handling their initialization and inter-agent communication.
/// 
/// The `ColonyManager` acts as the primary registry for the five Pippo identities 
/// and provides methods for submitting votes and querying shared trades.
pub struct ColonyManager {
    agents: Vec<Arc<dyn TradingPersonality>>,
}

impl ColonyManager {
    /// Initializes a new `ColonyManager` with all five standard Pippo variants.
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

    /// Returns a slice of all registered agents.
    pub fn agents(&self) -> &[Arc<dyn TradingPersonality>] {
        &self.agents
    }

    /// Submits a vote on behalf of an agent for a specific market.
    /// 
    /// currently this merely logs the vote and provides a placeholder for 
    /// future SQLite integration.
    pub async fn submit_vote(&self, agent_id: &str, market_id: &str, vote: bool) -> Result<()> {
        info!("Agent {} voted {} on market {}", agent_id, if vote { "YES" } else { "NO" }, market_id);
        // TODO: Store vote in shared SQLite table
        Ok(())
    }

    /// Retrieves the list of trades across the entire colony for inter-agent visibility.
    pub async fn get_colony_trades(&self) -> Result<Vec<String>> {
        // TODO: Query shared trades table for visibility
        Ok(vec![])
    }
}
