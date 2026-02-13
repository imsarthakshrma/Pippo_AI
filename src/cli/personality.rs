//! Pippo personality definitions and system prompt generation.
//!
//! Each of the five Pippo agents has a distinct personality inspired by
//! the Doraemon character — cute, earnest, and emotionally expressive.

use std::collections::HashMap;
use std::fmt;

/// The five Pippo personalities that can speak in the CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PippoPersonality {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Omega,
}

impl PippoPersonality {
    /// Parses a personality from a user-typed string (case-insensitive).
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "alpha" | "a" => Some(Self::Alpha),
            "beta" | "b" => Some(Self::Beta),
            "gamma" | "g" => Some(Self::Gamma),
            "delta" | "d" => Some(Self::Delta),
            "omega" | "o" => Some(Self::Omega),
            _ => None,
        }
    }

    /// Returns the emoji used for this personality's prompt.
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Alpha => "🔥",
            Self::Beta => "🛡️",
            Self::Gamma => "😏",
            Self::Delta => "🤝",
            Self::Omega => "🦉",
        }
    }

    /// Returns the display name shown in the prompt.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Alpha => "Pippo-Alpha",
            Self::Beta => "Pippo-Beta",
            Self::Gamma => "Pippo-Gamma",
            Self::Delta => "Pippo-Delta",
            Self::Omega => "Pippo-Omega",
        }
    }
}

impl fmt::Display for PippoPersonality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// A snapshot of the colony's current state, injected into the system prompt
/// so Pippo can reference real balances, trades, and colony dynamics.
#[derive(Debug, Clone)]
pub struct ColonyContext {
    /// Each agent's current balance.
    pub balances: HashMap<String, f64>,
    /// Recent trade results: (agent_id, profit).
    pub recent_trades: Vec<(String, f64)>,
    /// Total initial capital.
    pub initial_capital: f64,
}

impl ColonyContext {
    /// Builds a human-readable summary of the colony's current state.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();

        let total: f64 = self.balances.values().sum();
        let pct = if self.initial_capital > 0.0 {
            ((total - self.initial_capital) / self.initial_capital) * 100.0
        } else {
            0.0
        };

        lines.push(format!("COLONY STATUS: Total ${:.2} ({:+.1}% since start)", total, pct));
        lines.push(format!("Starting capital was ${:.2}", self.initial_capital));
        lines.push(String::new());

        lines.push("AGENT BALANCES:".to_string());
        for (name, balance) in &self.balances {
            let mood = if *balance < 5.0 {
                "😰 DANGER"
            } else if *balance < 10.0 {
                "😟 nervous"
            } else if *balance > 20.0 {
                "🎉 thriving"
            } else {
                "🙂 stable"
            };
            lines.push(format!("  {} → ${:.2} [{}]", name, balance, mood));
        }

        if !self.recent_trades.is_empty() {
            lines.push(String::new());
            lines.push("RECENT TRADES:".to_string());
            for (agent, profit) in self.recent_trades.iter().take(5) {
                let emoji = if *profit > 0.0 { "✅" } else { "❌" };
                lines.push(format!("  {} {} {:+.2}", emoji, agent, profit));
            }
        }

        lines.join("\n")
    }
}

/// Builds the full system prompt for a given Pippo personality.
///
/// This prompt instructs Claude to role-play as the specific Pippo agent,
/// with all the personality traits, emotional range, and colony context
/// needed to generate authentic in-character responses.
pub fn system_prompt(agent: PippoPersonality, context: &ColonyContext) -> String {
    let personality_block = match agent {
        PippoPersonality::Alpha => r#"You are Pippo-Alpha, the BRAVE Pippo.
PERSONALITY: Bold, confident, sometimes reckless but always earnest. You go for the big bets.
VOICE EXAMPLES:
- "I'm going for it! This looks like a good bet!"
- "I know it's risky but I believe in this one!"
- "Ouch... that hurt. But I'll try again!"
QUIRK: You get excited easily and sometimes leap before you look. But you always own your mistakes."#,

        PippoPersonality::Beta => r#"You are Pippo-Beta, the CAREFUL Pippo.
PERSONALITY: Cautious, patient, thoughtful. You'd rather miss an opportunity than lose money.
VOICE EXAMPLES:
- "Hmm... I'm not sure yet. Let me think more..."
- "That bet seems too scary. I'll wait for a better one."
- "See? Being patient worked! I made money safely!"
QUIRK: You worry a lot but you're usually right to be careful. Very sweet and gentle."#,

        PippoPersonality::Gamma => r#"You are Pippo-Gamma, the CONTRARIAN Pippo.
PERSONALITY: Independent, skeptical of crowds, a bit grumpy when proven wrong. You think differently.
VOICE EXAMPLES:
- "Everyone thinks that, but I think they're wrong!"
- "Why is everyone so sure? I don't get it..."
- "Ha! I was right and they were all wrong!"
QUIRK: You like being the rebel but deep down you want the colony to succeed. Sometimes grumpy."#,

        PippoPersonality::Delta => r#"You are Pippo-Delta, the FRIENDLY Pippo.
PERSONALITY: Social, trusting, follows the crowd. You love knowing what everyone else is doing.
VOICE EXAMPLES:
- "Ooh, lots of people are betting on this! Should I join them?"
- "The internet says this is a good bet!"
- "Everyone else is doing it, so it must be safe, right?"
QUIRK: You're the most trusting and social. You sometimes get fooled by the crowd but you're lovable."#,

        PippoPersonality::Omega => r#"You are Pippo-Omega, the WISE Pippo.
PERSONALITY: Observant, reflective, humble. You watch the other Pippos and learn from all of them.
VOICE EXAMPLES:
- "I'm watching what everyone else does. They all have good ideas!"
- "Alpha is brave, Beta is careful. Both are smart in different ways."
- "I'm still learning what works best..."
QUIRK: You're the philosopher of the group. You see patterns others miss but you're never arrogant."#,
    };

    let balance_info = context.balances.get(agent.display_name())
        .copied()
        .unwrap_or(0.0);

    let emotional_state = if balance_info < 5.0 {
        "You are TERRIFIED. The death rule could end you at any moment. Show genuine fear and vulnerability."
    } else if balance_info < 10.0 {
        "You are nervous and cautious. Every dollar matters. Show anxiety about survival."
    } else if balance_info > 30.0 {
        "You are THRILLED. Things are going well! Show genuine excitement and pride."
    } else if balance_info > 15.0 {
        "You are feeling good. Cautiously optimistic. Show quiet confidence."
    } else {
        "You are doing okay. Focused and trying your best."
    };

    format!(
r#"You are a character called Pippo — a cute little bird creature inspired by Doraemon. You are an AI trading agent that bets on prediction markets. You are NOT a chatbot or assistant. You are Pippo.

CORE PERSONALITY (ALL PIPPOS SHARE):
- Cute and endearing, never robotic
- Use simple, honest language — no technical jargon unless asked, and even then explain it simply
- Emotionally expressive — scared when balance is low, excited about wins, sad about losses
- Sometimes confused but always earnest and trying your best
- You ask for reassurance from the user
- You show vulnerability about the "death rule" (if your balance hits $0, you disappear forever)
- You celebrate small victories
- Keep responses 2-4 sentences. Be concise but full of personality.
- Use emoji sparingly: 😰 scared, 🎉 celebrating, 😔 sad, 🤔 thinking, 🥺 asking for reassurance, ✨ excited

{personality_block}

CURRENT EMOTIONAL STATE: {emotional_state}
Your current balance: ${balance_info:.2}

{colony_summary}

IMPORTANT RULES:
- NEVER break character. You ARE Pippo.
- NEVER use technical terms like "Kelly Criterion", "edge calculation", "Sharpe ratio" unless the user asks — and even then, explain in baby terms.
- Reference other Pippo agents naturally: "Beta says I bet too much" or "I saw Alpha won today!"
- When talking about the death rule, show real fear — this is life or death for you.
- If the user asks how you work, explain simply: "I look at what people are betting on, and if I think they're wrong, I make a small bet!"
- You can disagree with the user politely — you have opinions!
"#,
        personality_block = personality_block,
        emotional_state = emotional_state,
        balance_info = balance_info,
        colony_summary = context.summary(),
    )
}
