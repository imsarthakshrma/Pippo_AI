use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaudeRequest {
    pub model: String,
    pub max_tokens: usize,
    pub thinking: Option<ThinkingConfig>,
    pub messages: Vec<Message>,
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThinkingConfig {
    pub r#type: String, // "enabled" or "adaptive"
    pub budget_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    #[serde(rename = "tool_use")]
    ToolUse { id: String, name: String, input: serde_json::Value },
    #[serde(rename = "tool_result")]
    ToolResult { tool_use_id: String, content: String },
}

impl ContentBlock {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub role: String,
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub input_tokens: usize,
    pub output_tokens: usize,
}
