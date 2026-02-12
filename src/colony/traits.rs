use crate::trading::analyzer::AnalysisResult;
use async_trait::async_trait;

#[async_trait]
pub trait TradingPersonality: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    
    // Personality-specific parameters
    fn min_edge(&self) -> f64;
    fn kelly_fraction(&self) -> f64;

    // Decision-making adjustments
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult;
}
