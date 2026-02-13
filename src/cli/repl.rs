//! The interactive REPL for chatting with Pippo agents.
//!
//! Supports multi-turn conversation, personality switching,
//! and live colony status display.

use crate::cli::personality::{ColonyContext, PippoPersonality, system_prompt};
use crate::claude::client::ClaudeClient;
use crate::claude::types::*;
use anyhow::Result;
use std::io::{self, BufRead, Write};

/// Maintains conversation state across turns with a specific Pippo personality.
pub struct PippoChat {
    client: ClaudeClient,
    context: ColonyContext,
    active: PippoPersonality,
    history: Vec<Message>,
    model: String,
    max_tokens: usize,
}

impl PippoChat {
    /// Creates a new `PippoChat` session.
    pub fn new(
        client: ClaudeClient,
        context: ColonyContext,
        model: String,
        max_tokens: usize,
    ) -> Self {
        Self {
            client,
            context,
            active: PippoPersonality::Alpha,
            history: Vec::new(),
            model,
            max_tokens,
        }
    }

    /// Switches to a different Pippo personality and clears conversation history.
    fn switch_personality(&mut self, personality: PippoPersonality) {
        self.active = personality;
        self.history.clear();
        println!();
        println!("  {} Switched to {}! {}", 
            personality.emoji(), 
            personality.display_name(),
            personality.emoji()
        );
        println!();
    }

    /// Prints the current colony status in a friendly format.
    fn print_status(&self) {
        println!();
        println!("  ╭─────────────────────────────────────╮");
        println!("  │   🐥 Colony Status                  │");
        println!("  ╰─────────────────────────────────────╯");
        for line in self.context.summary().lines() {
            println!("  {}", line);
        }
        println!();
    }

    /// Sends a user message to Claude and returns the Pippo-flavored response.
    async fn send_message(&mut self, user_text: &str) -> Result<String> {
        // Add user message to history
        self.history.push(Message {
            role: "user".to_string(),
            content: vec![ContentBlock::Text { text: user_text.to_string() }],
        });

        let request = ClaudeRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            thinking: None, // No extended thinking for chat — pure personality
            messages: self.history.clone(),
            system: Some(system_prompt(self.active, &self.context)),
            tools: None,
        };

        let response = self.client.send_request(&request).await?;

        // Extract text blocks from response
        let text: String = response.content.iter()
            .filter_map(|b| b.as_text())
            .collect::<Vec<_>>()
            .join(" ");

        // Add assistant response to history
        self.history.push(Message {
            role: "assistant".to_string(),
            content: response.content,
        });

        // Keep history manageable (last 20 turns = 10 exchanges)
        if self.history.len() > 20 {
            self.history.drain(0..2);
        }

        Ok(text)
    }
}

/// Runs the interactive Pippo REPL.
///
/// This is the main entry point for `cargo run -- --chat`.
pub async fn run_repl(chat: &mut PippoChat) -> Result<()> {
    print_welcome(chat.active);

    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    loop {
        // Print prompt
        print!("  🐥 {}> ", chat.active.display_name());
        io::stdout().flush()?;

        // Read input
        let line = match reader.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e.into()),
            None => break, // EOF
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Handle special commands
        if trimmed.starts_with('/') {
            match handle_command(chat, trimmed) {
                CommandResult::Continue => continue,
                CommandResult::Quit => break,
                CommandResult::NotACommand => {} // Fall through to send as message
            }
        }

        // Send to Claude and print response
        match chat.send_message(trimmed).await {
            Ok(response) => {
                println!();
                println!("  {} {}", chat.active.emoji(), response);
                println!();
            }
            Err(e) => {
                println!();
                println!("  😰 Oh no... I couldn't think properly. (Error: {})", e);
                println!();
            }
        }
    }

    println!();
    println!("  🐥 Bye bye! I'll keep trading while you're gone! 💛");
    println!();
    Ok(())
}

enum CommandResult {
    Continue,
    Quit,
    NotACommand,
}

fn handle_command(chat: &mut PippoChat, input: &str) -> CommandResult {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();

    match cmd.as_str() {
        "/quit" | "/q" | "/exit" => CommandResult::Quit,

        "/status" | "/s" => {
            chat.print_status();
            CommandResult::Continue
        }

        "/switch" => {
            if parts.len() < 2 {
                println!();
                println!("  Usage: /switch <alpha|beta|gamma|delta|omega>");
                println!("  Shortcuts: /switch a, /switch b, etc.");
                println!();
                return CommandResult::Continue;
            }
            match PippoPersonality::from_str_loose(parts[1]) {
                Some(p) => {
                    chat.switch_personality(p);
                }
                None => {
                    println!();
                    println!("  🤔 I don't know a Pippo called \"{}\"...", parts[1]);
                    println!("  Try: alpha, beta, gamma, delta, or omega");
                    println!();
                }
            }
            CommandResult::Continue
        }

        "/help" | "/h" => {
            print_help();
            CommandResult::Continue
        }

        _ => CommandResult::NotACommand,
    }
}

fn print_welcome(personality: PippoPersonality) {
    println!();
    println!("  ╭──────────────────────────────────────────────╮");
    println!("  │                                              │");
    println!("  │      🐥  Welcome to Pippo Colony Chat!  🐥   │");
    println!("  │                                              │");
    println!("  │  Talk to the colony. They're all here.       │");
    println!("  │  Type /help for commands.                    │");
    println!("  │                                              │");
    println!("  ╰──────────────────────────────────────────────╯");
    println!();
    println!("  {} {} is ready to chat!", personality.emoji(), personality.display_name());
    println!();
}

fn print_help() {
    println!();
    println!("  ╭─────────────────────────────────────╮");
    println!("  │   🐥 Pippo Chat Commands            │");
    println!("  ╰─────────────────────────────────────╯");
    println!("  /switch <name>  Switch personality (alpha, beta, gamma, delta, omega)");
    println!("  /status         Show colony balances and recent trades");
    println!("  /help           Show this help");
    println!("  /quit           Exit chat");
    println!();
}
