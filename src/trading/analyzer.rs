use crate::claude::{ClaudeClient, ClaudeRequest, Message, ContentBlock, ThinkingConfig};
use crate::market::Market;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MarketAnalyzerTrait: Send + Sync {
    async fn analyze_markets(&self, markets: &[Market], budget: usize) -> Result<Vec<AnalysisResult>>;
}

/// Responsible for analyzing prediction markets using Claude's reasoning capabilities.
pub struct MarketAnalyzer {
    client: ClaudeClient,
}

impl MarketAnalyzer {
    /// Creates a new `MarketAnalyzer` with an initialized Claude client.
    pub fn new(api_key: String) -> Self {
        Self {
            client: ClaudeClient::new(api_key),
        }
    }
}

#[async_trait]
impl MarketAnalyzerTrait for MarketAnalyzer {
    /// Analyzes a list of markets and returns fair value estimates via Claude.
    async fn analyze_markets(&self, markets: &[Market], budget: usize) -> Result<Vec<AnalysisResult>> {
        let system_prompt = "You are Pippo, an autonomous prediction market trading agent. \
            Your goal is to grow your capital or shut down forever. \
            Analyze the following markets deeply. Identify mispricing (>8% edge). \
            Use your extended thinking budget for complex probability calculations and risk assessment. \
            Respond ONLY with a JSON array of objects containing 'market_id', 'fair_value', 'edge', and 'rationale'.";

        let market_data = serde_json::to_string(markets)?;
        
        let request = ClaudeRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 16000,
            thinking: Some(ThinkingConfig {
                r#type: "enabled".to_string(),
                budget_tokens: budget,
            }),
            system: Some(system_prompt.to_string()),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![ContentBlock::Text {
                    text: format!("Here are the active markets: {}", market_data),
                }],
            }],
            tools: None, // Tools can be added here once defined
        };

        let response = self.client.chat_with_tools(request).await?;
        
        // Extract text from response and parse JSON
        for block in response.content {
            if let ContentBlock::Text { text } = block {
                if let Ok(results) = serde_json::from_str::<Vec<AnalysisResult>>(&text) {
                    return Ok(results);
                }
            }
        }

        Err(anyhow::anyhow!("Failed to parse analysis results from Claude response"))
    }
}

/// The raw output from a market analysis session.
#[derive(Debug, serde::Deserialize)]
pub struct AnalysisResult {
    /// ID of the scrutinized market.
    pub market_id: String,
    /// Model's estimated fair value (probability between 0 and 1).
    pub fair_value: f64,
    /// Calculated advantage over the current market price.
    pub edge: f64,
    /// Summary of the model's reasoning for this valuation.
    pub rationale: String,
}
