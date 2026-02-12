use crate::trading::analyzer::AnalysisResult;
use async_trait::async_trait;

/// Defines the behavior and risk parameters for a specific agent personality.
/// 
/// Each implementer of this trait brings a unique perspective to market analysis, 
/// adjusting baseline findings according to their specific "psychology" 
/// (e.g., more or less cautious, contrarian, etc.).
#[async_trait]
pub trait TradingPersonality: Send + Sync {
    /// Returns the name of the identity (e.g., "Alpha", "Omega").
    fn name(&self) -> &str;
    /// Provides a brief summary of the agent's trading philosophy.
    fn description(&self) -> &str;
    
    /// The minimum required edge (expected value) before the agent considers a trade.
    fn min_edge(&self) -> f64;
    /// The fraction of the Kelly Criterion to apply (used for risk management).
    fn kelly_fraction(&self) -> f64;

    /// Allows the personality to modify or filter a baseline analysis result.
    /// 
    /// This is where traits like "Contrarian" or "Momentum" are applied to the 
    /// raw probabilistic findings from the `ClaudeAnalyzer`.
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult;
}
