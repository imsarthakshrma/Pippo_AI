use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A comprehensive report detailing an agent's performance during a backtest.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BacktestReport {
    /// Unique identifier for the agent (e.g., "alpha", "beta").
    pub agent_id: String,
    /// Total percentage return on capital (set externally by the simulator).
    pub total_return_percent: f64,
    /// Ratio of winning trades to total trades.
    pub win_rate: f64,
    /// Maximum observed peak-to-trough decline in capital.
    pub max_drawdown: f64,
    /// Risk-adjusted return metric.
    pub sharpe_ratio: f64,
    /// Total number of trades executed during the period.
    pub trade_count: usize,
    /// Number of winning trades (profit > 0).
    pub win_count: usize,
    /// Cumulative raw profit/loss across all trades.
    pub total_raw_profit: f64,
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
    pub fn track_trade(&mut self, agent_id: &str, profit: f64) {
        let report = self.reports.entry(agent_id.to_string()).or_insert_with(|| BacktestReport {
            agent_id: agent_id.to_string(),
            ..Default::default()
        });

        report.trade_count += 1;
        if profit > 0.0 {
            report.win_count += 1;
        }
        report.win_rate = report.win_count as f64 / report.trade_count as f64;
        report.total_raw_profit += profit;
    }

    /// Collects all current metrics into a list of reports.
    pub fn generate_reports(&self) -> Vec<BacktestReport> {
        self.reports.values().cloned().collect()
    }
}
