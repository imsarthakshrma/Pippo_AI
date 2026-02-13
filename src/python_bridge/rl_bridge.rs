//! Bridge for Reinforcement Learning tasks handled in Python.

use pyo3::prelude::*;
use crate::python_bridge::init_python;
use anyhow::Result;
use serde_json::Value;

/// Calls the Python RL system to calculate a reward for a settled trade.
pub fn calculate_reward(trade_id: &str, outcome: &str) -> Result<f64> {
    init_python();
    Python::with_gil(|py| {
        let rl_module = py.import("rl.rewards")?;
        let reward: f64 = rl_module
            .getattr("calculate_trade_reward")?
            .call1((trade_id, outcome))?
            .extract()?;
        Ok(reward)
    })
    .map_err(|e| anyhow::anyhow!("Python RL error: {}", e))
}

/// Calls the Python RL system to suggest an optimized position size.
pub fn suggest_position_size(market_id: &str, current_balance: f64) -> Result<f64> {
    init_python();
    Python::with_gil(|py| {
        let rl_module = py.import("rl.agent")?;
        let size: f64 = rl_module
            .getattr("get_optimized_size")?
            .call1((market_id, current_balance))?
            .extract()?;
        Ok(size)
    })
    .map_err(|e| anyhow::anyhow!("Python RL error: {}", e))
}
