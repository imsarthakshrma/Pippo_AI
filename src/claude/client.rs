use crate::claude::types::*;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use tracing::info;

pub struct ClaudeClient {
    client: reqwest::Client,
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("anthropic-beta", HeaderValue::from_static("interleaved-thinking-2025-05-14"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client, api_key }
    }

    pub async fn chat_with_tools(&self, mut request: ClaudeRequest) -> Result<ClaudeResponse> {
        loop {
            let response = self.send_request(&request).await?;
            
            // Register this response's content in the message history for the next potential turn
            request.messages.push(Message {
                role: response.role.clone(),
                content: response.content.clone(),
            });

            // Check if there are tool uses in the response
            let tool_uses: Vec<_> = response.content.iter().filter_map(|block| {
                if let ContentBlock::ToolUse { id, name, input } = block {
                    Some((id.clone(), name.clone(), input.clone()))
                } else {
                    None
                }
            }).collect();

            if tool_uses.is_empty() {
                return Ok(response);
            }

            // Execute tools and collect results
            let mut tool_results = Vec::new();
            for (id, name, input) in tool_uses {
                info!("Claude requested tool use: {} with input: {}", name, input);
                // TODO: Dispatch to actual tool implementation
                let result = self.execute_tool(&name, input).await?;
                tool_results.push(ContentBlock::ToolResult {
                    tool_use_id: id,
                    content: result,
                });
            }

            // Push tool results as a new message from 'user'
            request.messages.push(Message {
                role: "user".to_string(),
                content: tool_results,
            });

            // The loop continues, sending the enriched history (including thinking blocks) back to Claude
        }
    }

    async fn execute_tool(&self, name: &str, _input: serde_json::Value) -> Result<String> {
        match name {
            "fetch_polymarket_data" => Ok("Sample data result".to_string()),
            _ => Ok(format!("Tool {} not implemented", name)),
        }
    }

    pub async fn send_request(&self, request: &ClaudeRequest) -> Result<ClaudeResponse> {
        let url = "https://api.anthropic.com/v1/messages";
        let response = self.client.post(url)
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow::anyhow!("Claude API error: {}", err_text));
        }

        let cl_response = response.json::<ClaudeResponse>().await?;
        Ok(cl_response)
    }
}
