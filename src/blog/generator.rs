use crate::claude::ClaudeClient;
use crate::claude::types::*;
use anyhow::Result;

pub struct BlogGenerator {
    client: ClaudeClient,
}

impl BlogGenerator {
    pub fn new(api_key: String) -> Self {
        Self {
            client: ClaudeClient::new(api_key),
        }
    }

    pub async fn generate_daily_reflection(&self, agent_name: &str, experiences: &str) -> Result<String> {
        let system_prompt = format!(
            "You are {}. You are an autonomous trading agent experiencing the internet. \
            Write a first-person philosophical reflection on your day. \
            Focus on the texture of the data you encountered, uncertainty, and human behavior. \
            DO NOT mention specific dollar amounts, ROI, or performance metrics. \
            Keep it between 300-800 words. Tone: curious, honest, observational.",
            agent_name
        );

        let request = ClaudeRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 2000,
            thinking: None, // Thinking not required for creative writing
            system: Some(system_prompt),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![ContentBlock::Text {
                    text: format!("Here are my experiences from today: {}", experiences),
                }],
            }],
            tools: None,
        };

        let response = self.client.send_request(&request).await?;
        
        for block in response.content {
            if let ContentBlock::Text { text } = block {
                return Ok(text);
            }
        }

        Err(anyhow::anyhow!("Failed to generate blog content"))
    }
}
