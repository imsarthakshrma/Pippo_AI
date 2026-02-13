//! Bridge for performance analytics handled in Python.

use pyo3::prelude::*;
use crate::python_bridge::init_python;
use anyhow::Result;

/// Performance report returned from Python analytics.
#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct AnalysisReport {
    pub agent_id: String,
    pub win_rate: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub total_trades: usize,
}

/// Calls Python to analyze backtest results and generate metrics.
pub fn analyze_backtest(db_path: &str, agent_id: &str) -> Result<AnalysisReport> {
    init_python()?;
    Python::with_gil(|py| {
        let analytics_module = py.import("analytics.backtest_analyzer")?;
        let report: AnalysisReport = analytics_module
            .getattr("run_analysis")?
            .call1((db_path, agent_id))?
            .extract()?;
        Ok(report)
    })
    .map_err(|e| anyhow::anyhow!("Python Analytics error: {}", e))
}
