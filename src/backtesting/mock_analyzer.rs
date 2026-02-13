use crate::trading::analyzer::{MarketAnalyzerTrait, AnalysisResult};
use crate::market::Market;
use anyhow::Result;
use async_trait::async_trait;

/// A fast, hardcoded analyzer for backtesting.
pub struct MockAnalyzer;

#[async_trait]
impl MarketAnalyzerTrait for MockAnalyzer {
    async fn analyze_markets(&self, markets: &[Market], _budget: usize) -> Result<Vec<AnalysisResult>> {
        let mut results = Vec::new();
        for market in markets {
            // Predict a 10% edge based on the first outcome probability
            let fair_value = if !market.current_odds.is_empty() {
                (market.current_odds[0] + 0.10).min(0.95)
            } else {
                0.5
            };
            
            results.push(AnalysisResult {
                market_id: market.id.clone(),
                fair_value,
                edge: fair_value - market.current_odds.get(0).cloned().unwrap_or(0.5),
                rationale: "Mock analysis: identifying slight mispricing for backtest demo.".to_string(),
            });
        }
        Ok(results)
    }
}
