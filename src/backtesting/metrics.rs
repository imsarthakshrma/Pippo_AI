use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A comprehensive report detailing an agent's performance during a backtest.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BacktestReport {
    /// Unique identifier for the agent (e.g., "alpha", "beta").
    pub agent_id: String,
    /// Total percentage return on capital.
    pub total_return_percent: f64,
    /// Ratio of winning trades to total trades.
    pub win_rate: f64,
    /// Maximum observed peak-to-trough decline in capital.
    pub max_drawdown: f64,
    /// Risk-adjusted return metric.
    pub sharpe_ratio: f64,
    /// Total number of trades executed during the period.
    pub trade_count: usize,
    /// Whether the agent met the "graduation" criteria (Sharpe > 1.5, Drawdown < 40%).
    pub success: bool,
}

/// Centralizes the tracking and calculation of performance metrics for all agents.
pub struct MetricsTracker {
    reports: HashMap<String, BacktestReport>,
}

impl MetricsTracker {
    /// Creates a new `MetricsTracker`.
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
        }
    }

    /// Records a new trade result for an agent and updates cumulative metrics.
    pub fn track_trade(&mut self, _agent_id: &str, _profit: f64) {
        // TODO: Update metrics for the agent
    }

    /// Collects all current metrics into a list of reports.
    pub fn generate_reports(&self) -> Vec<BacktestReport> {
        self.reports.values().cloned().collect()
    }
}
