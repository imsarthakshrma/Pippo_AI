use crate::colony::traits::TradingPersonality;
use crate::trading::analyzer::AnalysisResult;
use async_trait::async_trait;

/// Personality: Aggressive trader with a low edge threshold and high risk appetite.
pub struct PippoAlpha;
#[async_trait]
impl TradingPersonality for PippoAlpha {
    fn name(&self) -> &str { "Pippo-Alpha" }
    fn description(&self) -> &str { "Aggressive: High Kelly, low edge threshold." }
    fn min_edge(&self) -> f64 { 0.08 }
    fn kelly_fraction(&self) -> f64 { 0.06 }
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult { analysis }
}

/// Personality: Cautious trader requiring significant edge and using smaller position sizes.
pub struct PippoBeta;
#[async_trait]
impl TradingPersonality for PippoBeta {
    fn name(&self) -> &str { "Pippo-Beta" }
    fn description(&self) -> &str { "Conservative: High edge threshold, low Kelly." }
    fn min_edge(&self) -> f64 { 0.12 }
    fn kelly_fraction(&self) -> f64 { 0.03 }
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult { analysis }
}

/// Personality: Contrarian trader that seeks to identify and fade crowd bias.
pub struct PippoGamma;
#[async_trait]
impl TradingPersonality for PippoGamma {
    fn name(&self) -> &str { "Pippo-Gamma" }
    fn description(&self) -> &str { "Contrarian: Fades crowds, seeks unpopular value." }
    fn min_edge(&self) -> f64 { 0.10 }
    fn kelly_fraction(&self) -> f64 { 0.04 }
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult { 
        // Logic to prioritize contrarian plays could be added here
        analysis 
    }
}

/// Personality: Momentum trader that follows strong social and market trends.
pub struct PippoDelta;
#[async_trait]
impl TradingPersonality for PippoDelta {
    fn name(&self) -> &str { "Pippo-Delta" }
    fn description(&self) -> &str { "Momentum: Follows trends and sentiment-driven moves." }
    fn min_edge(&self) -> f64 { 0.09 }
    fn kelly_fraction(&self) -> f64 { 0.05 }
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult { analysis }
}

/// Personality: Meta-learner that observes colleague behavior to refine collective state.
pub struct PippoOmega;
#[async_trait]
impl TradingPersonality for PippoOmega {
    fn name(&self) -> &str { "Pippo-Omega" }
    fn description(&self) -> &str { "Meta-learner: Observational and learning-oriented." }
    fn min_edge(&self) -> f64 { 0.10 }
    fn kelly_fraction(&self) -> f64 { 0.04 }
    async fn adjust_analysis(&self, analysis: AnalysisResult) -> AnalysisResult { analysis }
}
