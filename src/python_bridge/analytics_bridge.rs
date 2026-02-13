//! Bridge for performance analytics handled in Python.

use pyo3::prelude::*;
use crate::python_bridge::init_python;
use anyhow::Result;

/// Calls Python to analyze backtest results and generate metrics.
pub fn analyze_backtest(db_path: &str, agent_id: &str) -> Result<String> {
    init_python();
    Python::with_gil(|py| {
        let analytics_module = py.import("analytics.backtest_analyzer")?;
        let report: String = analytics_module
            .getattr("run_analysis")?
            .call1((db_path, agent_id))?
            .extract()?;
        Ok(report)
    })
    .map_err(|e| anyhow::anyhow!("Python Analytics error: {}", e))
}
