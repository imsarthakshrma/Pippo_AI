use crate::colony::traits::TradingPersonality;
use crate::colony::variants::*;
use sqlx::SqlitePool;
use std::sync::Arc;
use anyhow::Result;
use tracing::info;

/// Manages the collection of agents, handling their initialization and inter-agent communication.
pub struct ColonyManager {
    agents: Vec<Arc<dyn TradingPersonality>>,
    pool: SqlitePool,
}

impl ColonyManager {
    /// Initializes a new `ColonyManager`.
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            agents: vec![
                Arc::new(PippoAlpha),
                Arc::new(PippoBeta),
                Arc::new(PippoGamma),
                Arc::new(PippoDelta),
                Arc::new(PippoOmega),
            ],
            pool,
        }
    }

    /// Returns a slice of all registered agents.
    pub fn agents(&self) -> &[Arc<dyn TradingPersonality>] {
        &self.agents
    }

    /// Submits a vote on behalf of an agent for a specific market to the shared database.
    pub async fn submit_vote(&self, agent_id: &str, market_id: &str, vote: &crate::colony::voting_protocol::Vote, trigger: &crate::colony::voting_protocol::VoteTrigger) -> Result<()> {
        let trigger_str = format!("{:?}", trigger);
        let pos_str = format!("{:?}", vote.position);

        sqlx::query(
            "INSERT INTO colony_votes (id, market_id, agent_id, trigger_reason, position, confidence, reasoning_hash, edge_estimate) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(market_id)
        .bind(agent_id)
        .bind(trigger_str)
        .bind(pos_str)
        .bind(vote.confidence)
        .bind(&vote.reasoning_hash)
        .bind(vote.edge_estimate)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Retrieves the list of recent trade results for inter-agent visibility.
    pub async fn get_colony_trades(&self) -> Result<Vec<(String, f64)>> {
        let trades = sqlx::query_as::<_, (String, f64)>("SELECT agent_id, reward_value FROM rl_rewards_detailed ORDER BY timestamp DESC LIMIT 50")
            .fetch_all(&self.pool)
            .await?;
        Ok(trades)
    }
}
