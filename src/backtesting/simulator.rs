use crate::backtesting::data_collector::HistoricalMarket;
use crate::trading::analyzer::{MarketAnalyzerTrait, AnalysisResult};
use crate::colony::colony_manager::ColonyManager;
use crate::logic::RlSystem;
use crate::market::Market;
use crate::backtesting::metrics::MetricsTracker;
use crate::colony::voting_protocol::{VotingRound, VoteTrigger, Vote, VotePosition, VotingOutcome};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Duration, Utc};

/// Sentinel for markets with no defined end date — treated as perpetually active.
const FAR_FUTURE: DateTime<Utc> = DateTime::<Utc>::MAX_UTC;
use tracing::{info, warn};
use std::sync::Arc;
use sqlx::SqlitePool;
use std::collections::HashMap;

/// A single open trade within the backtest simulation.
#[derive(Debug, Clone)]
struct SimulatedPosition {
    market_id: String,
    agent_id: String,
    size: f64,
    entry_price: f64,
}

/// A market replayer that simulates a fixed period of time for backtesting agents.
pub struct BacktestSimulator {
    markets: Vec<HistoricalMarket>,
    current_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    pool: SqlitePool,
    colony: ColonyManager,
    analyzer: Arc<dyn MarketAnalyzerTrait>,
    rl: RlSystem,
    metrics: MetricsTracker,
    open_positions: Vec<SimulatedPosition>,
}

impl BacktestSimulator {
    /// Creates a new `BacktestSimulator`.
    pub fn new(
        markets: Vec<HistoricalMarket>, 
        start_at: DateTime<Utc>, 
        end_at: DateTime<Utc>,
        pool: SqlitePool,
        colony: ColonyManager,
        analyzer: Arc<dyn MarketAnalyzerTrait>,
        rl: RlSystem,
    ) -> Self {
        Self {
            markets,
            current_time: start_at,
            end_time: end_at,
            pool,
            colony,
            analyzer,
            rl,
            metrics: MetricsTracker::new(),
            open_positions: Vec::new(),
        }
    }

    /// Orchestrates the full simulation loop from start to end time.
    pub async fn run_simulation(&mut self) -> Result<()> {
        info!("Starting simulation from {} to {}", self.current_time, self.end_time);
        
        while self.current_time < self.end_time {
            if self.rl.check_death_rule() {
                warn!("Colony collapsed during backtest at {}. Settling {} open positions at entry cost.", self.current_time, self.open_positions.len());
                self.force_settle_open_positions().await?;
                break;
            }
            self.step().await?;
            self.current_time = self.current_time + Duration::minutes(10);
        }

        // Settle any positions still open after the time window expires
        if !self.open_positions.is_empty() {
            warn!("Simulation ended with {} open positions. Settling at entry cost.", self.open_positions.len());
            self.force_settle_open_positions().await?;
        }

        info!("Simulation completed. Total Return: {:.2}%", self.calculate_total_return());
        Ok(())
    }

    fn calculate_total_return(&self) -> f64 {
        if self.rl.total_initial_capital == 0.0 {
            return 0.0;
        }
        let current_capital: f64 = self.rl.agent_balances.values().sum();
        ((current_capital - self.rl.total_initial_capital) / self.rl.total_initial_capital) * 100.0
    }

    /// Executes a single 10-minute simulation step.
    async fn step(&mut self) -> Result<()> {
        // 1. Process resolved markets
        self.resolve_expiring_trades().await?;

        // 2. Identify active markets
        let active_historical = self.markets.iter()
            .filter(|m| m.created_at <= self.current_time && m.end_date.unwrap_or(FAR_FUTURE) > self.current_time)
            .collect::<Vec<_>>();

        if active_historical.is_empty() {
            return Ok(());
        }

        // 3. Resolve current prices
        let mut live_markets = Vec::new();
        for hm in active_historical {
            // Avoid duplicate trades on same market in same step or if already open
            if self.open_positions.iter().any(|p| p.market_id == hm.id) {
                continue;
            }

            let price = sqlx::query_as::<_, (f64,)>("SELECT price FROM historical_odds WHERE market_id = ? AND timestamp <= ? ORDER BY timestamp DESC LIMIT 1")
                .bind(&hm.id)
                .bind(self.current_time)
                .fetch_optional(&self.pool)
                .await?;

            if let Some((p,)) = price {
                live_markets.push(Market {
                    id: hm.id.clone(),
                    question: hm.question.clone(),
                    outcome_assets: vec!["Yes".to_string(), "No".to_string()],
                    current_odds: vec![p, 1.0 - p],
                });
            }
        }

        if live_markets.is_empty() {
            return Ok(());
        }

        // 4. Analyze markets
        let analysis_results = self.analyzer.analyze_markets(&live_markets, 0).await?;

        // 5. Colony processing & voting
        for res in analysis_results {
            let mut round = VotingRound {
                market_id: res.market_id.clone(),
                trigger_reason: VoteTrigger::HighStakes,
                participants: self.colony.agents().iter().map(|a| a.name().to_string()).collect(),
                votes: HashMap::new(),
                deadline: self.current_time + Duration::minutes(5),
            };

            for agent in self.colony.agents() {
                let adjusted = agent.adjust_analysis(AnalysisResult {
                    market_id: res.market_id.clone(),
                    fair_value: res.fair_value,
                    edge: res.edge,
                    rationale: res.rationale.clone(),
                }).await;

                if adjusted.edge >= agent.min_edge() {
                    let vote = Vote {
                        agent_id: agent.name().to_string(),
                        position: VotePosition::Yes,
                        confidence: 0.8,
                        reasoning_hash: "backtest_hash".to_string(),
                        edge_estimate: adjusted.edge,
                    };
                    
                    self.colony.submit_vote(agent.name(), &res.market_id, &vote, &round.trigger_reason).await?;
                    round.votes.insert(agent.name().to_string(), vote);
                }
            }

            if !round.votes.is_empty() {
                let outcome = round.resolve();
                if let VotingOutcome::Approved(_) = outcome {
                    // Logic: Follow the strongest approval for sizing (simplified for backtest)
                    if let Some(market) = live_markets.iter().find(|m| m.id == res.market_id) {
                        let price = market.current_odds[0];
                        for (agent_id, vote) in &round.votes {
                            if vote.position == VotePosition::Yes || vote.position == VotePosition::StrongYes {
                                let agent = self.colony.agents().iter().find(|a| a.name() == agent_id).unwrap();
                                let balance = *self.rl.agent_balances.get(agent_id).unwrap_or(&0.0);
                                let size = crate::trading::KellySizer::calculate_position_size(
                                    balance, 1.0 / price, res.fair_value, agent.kelly_fraction()
                                );

                                if size > 0.0 {
                                    self.open_positions.push(SimulatedPosition {
                                        market_id: res.market_id.clone(),
                                        agent_id: agent_id.clone(),
                                        size,
                                        entry_price: price,
                                    });
                                    // Deduct capital immediately
                                    self.rl.process_milestones(agent_id, -size);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn resolve_expiring_trades(&mut self) -> Result<()> {
        let mut i = 0;
        while i < self.open_positions.len() {
            let pos = &self.open_positions[i];
            let market = self.markets.iter()
                .find(|m| m.id == pos.market_id)
                .ok_or_else(|| anyhow!("Market {} not found for open position held by {}", pos.market_id, pos.agent_id))?;

            if let Some(end_date) = market.end_date {
                if self.current_time >= end_date {
                    let win = market.outcome.as_deref()
                        .map(|o| o.eq_ignore_ascii_case("yes"))
                        .unwrap_or(false);
                    let payout = if win { pos.size / pos.entry_price } else { 0.0 };
                    let profit = payout - pos.size;

                    info!("Trade resolved for {}: {} (Win: {}) P/L: ${:.2}", pos.agent_id, pos.market_id, win, profit);

                    self.rl.process_milestones(&pos.agent_id, payout);
                    self.metrics.track_trade(&pos.agent_id, profit);

                    self.open_positions.remove(i);
                    continue;
                }
            }
            i += 1;
        }
        Ok(())
    }

    /// Force-settles all remaining open positions at entry cost.
    ///
    /// Used when the colony collapses (Death Rule) or the simulation window ends
    /// with positions still open. Returns locked capital to each agent and records
    /// each forced settlement as a zero-profit trade.
    async fn force_settle_open_positions(&mut self) -> Result<()> {
        while let Some(pos) = self.open_positions.pop() {
            // Try to get the current mark from historical_odds; fall back to entry_price
            let mark = sqlx::query_as::<_, (f64,)>(
                "SELECT price FROM historical_odds WHERE market_id = ? AND timestamp <= ? ORDER BY timestamp DESC LIMIT 1"
            )
            .bind(&pos.market_id)
            .bind(self.current_time)
            .fetch_optional(&self.pool)
            .await?
            .map(|(p,)| p)
            .unwrap_or(pos.entry_price);

            // Mark-to-market P/L: what the position is worth now vs what was paid
            let current_value = pos.size * (mark / pos.entry_price);
            let profit = current_value - pos.size;

            warn!(
                "Force-settled {} position on {} for agent {} | Entry: {:.4} Mark: {:.4} P/L: ${:.2}",
                if profit >= 0.0 { "profitable" } else { "losing" },
                pos.market_id, pos.agent_id, pos.entry_price, mark, profit
            );

            // Return the marked value to the agent's balance
            self.rl.process_milestones(&pos.agent_id, current_value);
            self.metrics.track_trade(&pos.agent_id, profit);
        }
        Ok(())
    }
}
