use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BacktestReport {
    pub agent_id: String,
    pub total_return_percent: f64,
    pub win_rate: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub trade_count: usize,
    pub success: bool,
}

pub struct MetricsTracker {
    reports: HashMap<String, BacktestReport>,
}

impl MetricsTracker {
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
        }
    }

    pub fn track_trade(&mut self, _agent_id: &str, _profit: f64) {
        // TODO: Update metrics for the agent
    }

    pub fn generate_reports(&self) -> Vec<BacktestReport> {
        self.reports.values().cloned().collect()
    }
}
