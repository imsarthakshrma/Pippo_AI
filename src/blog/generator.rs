use anyhow::Result;
use crate::claude::{ClaudeClient, ClaudeRequest, Message, ContentBlock};
use crate::blog::prompts::*;

/// Generates personality-driven blog posts using the Claude API.
/// 
/// The `BlogGenerator` uses specific system prompts for each agent to create 
/// first-person reflections on daily market events and colony dynamics.
pub struct BlogGenerator {
    client: ClaudeClient,
}

impl BlogGenerator {
    /// Creates a new `BlogGenerator` with the provided Claude API key.
    pub fn new(api_key: String) -> Self {
        Self {
            client: ClaudeClient::new(api_key),
        }
    }

    /// Generates a daily blog post for a specific agent based on their day's events.
    pub async fn generate_daily_blog(
        &self,
        agent_id: &str,
        events: DayEvents,
    ) -> Result<String> {
        let system_prompt = match agent_id.to_lowercase().as_str() {
            "alpha" => ALPHA_SYSTEM_PROMPT,
            "beta" => BETA_SYSTEM_PROMPT,
            "gamma" => GAMMA_SYSTEM_PROMPT,
            "delta" => DELTA_SYSTEM_PROMPT,
            "omega" => OMEGA_SYSTEM_PROMPT,
            _ => return Err(anyhow::anyhow!("Unrecognized agent_id: {}", agent_id)),
        };

        let user_prompt = format!(
            "Today's experiences:\n{}\n\nData sources: {}\n\nColony moments: {}\n\nWrite your daily reflection.",
            events.main_events.join("\n"),
            events.data_sources.join("\n"),
            events.colony_moments.join("\n")
        );

        let request = ClaudeRequest {
            model: "claude-3-5-sonnet-20240620".to_string(), // Using Sonnet as requested for creative flow
            max_tokens: 2000,
            system: Some(system_prompt.to_string()),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![crate::claude::ContentBlock::Text { text: user_prompt }],
            }],
            tools: None,
            thinking: None, // No thinking mode for creative blogs
        };

        let response = self.client.send_request(&request).await?;
        let blog_content = response.content.first()
            .and_then(|c| c.as_text())
            .ok_or_else(|| anyhow::anyhow!("Claude failed to generate blog content (empty response)"))?
            .to_string();
            
        Ok(blog_content)
    }
}

/// Encapsulates the events and observations for a single agent's day.
pub struct DayEvents {
    /// Major world or market events the agent observed.
    pub main_events: Vec<String>,
    /// Specific data sources (e.g., NOAA, Polymarket) queried.
    pub data_sources: Vec<String>,
    /// Interactions or observations regarding other colony members.
    pub colony_moments: Vec<String>,
}
