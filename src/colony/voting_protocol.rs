use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VotingRound {
    pub market_id: String,
    pub trigger_reason: VoteTrigger,
    pub participants: Vec<String>,
    pub votes: HashMap<String, Vote>,
    pub deadline: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VoteTrigger {
    ResourceConflict,      // Multiple agents want same market
    HighStakes,            // Bet size >10% of balance
    EmergencyReview,       // Unusual situation
    WeeklyStrategy,        // Scheduled review
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote {
    pub agent_id: String,
    pub position: VotePosition,
    pub confidence: f64,       // 0.0 - 1.0
    pub reasoning_hash: String, // Hash of Claude's thinking (for privacy)
    pub edge_estimate: f64,    // What edge this agent sees
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VotePosition {
    StrongYes,    // High confidence, proceed with bet
    Yes,          // Moderate confidence, proceed
    Neutral,      // Abstain
    No,           // Don't take this bet
    StrongNo,     // High confidence, avoid
}

pub enum VotingOutcome {
    Vetoed,
    Approved(f64),
    DeferToHighestConfidence,
    DeferToOmega,
}

impl VotingRound {
    pub fn resolve(&self) -> VotingOutcome {
        // Rule 1: StrongNo from any agent = veto (safety first)
        if self.has_strong_no() {
            return VotingOutcome::Vetoed;
        }
        
        // Rule 2: Consensus (>3 agents agree) = proceed
        if self.consensus_count() >= 3 {
            return VotingOutcome::Approved(0.0); // Placeholder for aggregated size
        }
        
        // Rule 3: Split decision = highest confidence wins
        if self.is_split() {
            return VotingOutcome::DeferToHighestConfidence;
        }
        
        // Rule 4: Tie = defer to Pippo-Omega (meta-learner)
        return VotingOutcome::DeferToOmega;
    }

    fn has_strong_no(&self) -> bool {
        self.votes.values().any(|v| v.position == VotePosition::StrongNo)
    }

    fn consensus_count(&self) -> usize {
        self.votes.values()
            .filter(|v| v.position == VotePosition::Yes || v.position == VotePosition::StrongYes)
            .count()
    }

    fn is_split(&self) -> bool {
        let yes_count = self.consensus_count();
        let no_count = self.votes.values()
            .filter(|v| v.position == VotePosition::No || v.position == VotePosition::StrongNo)
            .count();
        yes_count > 0 && no_count > 0
    }
}
