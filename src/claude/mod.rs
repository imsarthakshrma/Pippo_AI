//! Client and types for the Anthropic Claude API.
//! 
//! This module provides a high-level `ClaudeClient` with support for 
//! tool-use loops and the specialized "extended thinking" blocks.

pub mod client;
pub mod types;

pub use client::ClaudeClient;
pub use types::*;
