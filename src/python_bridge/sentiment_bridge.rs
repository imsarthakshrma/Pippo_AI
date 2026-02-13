//! Bridge for sentiment analysis handled in Python.

use pyo3::prelude::*;
use crate::python_bridge::init_python;
use anyhow::Result;

/// Calls Python to analyze sentiment of a batch of text.
pub fn analyze_sentiment(texts: Vec<String>) -> Result<f64> {
    init_python()?;
    Python::with_gil(|py| {
        let sentiment_module = py.import("sentiment.analyzer")?;
        let score: f64 = sentiment_module
            .getattr("get_sentiment_score")?
            .call1((texts,))?
            .extract()?;
        Ok(score)
    })
    .map_err(|e: pyo3::PyErr| anyhow::anyhow!("Python Sentiment error: {}", e))
}
