use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Encapsulates a single round of voting among the colony agents.
/// 
/// Voting is triggered by specific events (high stakes, conflicts) and aims 
/// to reach a consensus or safety-based veto before executing a trade.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VotingRound {
    /// The ID of the market under consideration.
    pub market_id: String,
    /// What specific condition triggered this vote.
    pub trigger_reason: VoteTrigger,
    /// List of agent IDs participating in the vote.
    pub participants: Vec<String>,
    /// Map of agent ID to their specific vote.
    pub votes: HashMap<String, Vote>,
    /// Time limit for agents to submit their votes.
    pub deadline: DateTime<Utc>,
}

/// Categorizes why a voting round was initiated.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VoteTrigger {
    /// Multiple agents want to trade on the same market.
    ResourceConflict,
    /// A single bet size exceeds a safety threshold (e.g., >10% of balance).
    HighStakes,
    /// Manual or algorithmic emergency intervention.
    EmergencyReview,
    /// Routine strategy alignment session.
    WeeklyStrategy,
}

/// Details of an individual agent's vote.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote {
    /// The agent casting the vote.
    pub agent_id: String,
    /// The agent's stance on the proposal.
    pub position: VotePosition,
    /// The agent's confidence in its reasoning (0.0 to 1.0).
    pub confidence: f64,
    /// Cryptographic hash of the agent's internal reasoning block.
    pub reasoning_hash: String,
    /// The agent's estimate of the mathematical edge available in the trade.
    pub edge_estimate: f64,
}

/// Discrete positions an agent can take in a vote.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VotePosition {
    /// Strong commitment to proceed.
    StrongYes,
    /// Recommendation to proceed.
    Yes,
    /// No preference or missing information.
    Neutral,
    /// Recommendation to avoid.
    No,
    /// Absolute rejection (triggers immediate veto).
    StrongNo,
}

/// The final resolution of a voting round.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VotingOutcome {
    /// The trade was rejected (vetoed).
    Vetoed,
    /// The trade was approved with a specific aggregate confidence/size.
    Approved(f64),
    /// No consensus reached; follow the agent with the highest confidence.
    DeferToHighestConfidence,
    /// Standoff resolved by the Pippo-Omega meta-learner.
    DeferToOmega,
}

impl VotingRound {
    /// Resolves the voting round using prioritized colony rules:
    /// 1. Veto check (StrongNo)
    /// 2. Consensus check (>3 Yes)
    /// 3. Split check (Confidence comparison)
    /// 4. Tie resolution (Pippo-Omega)
    pub fn resolve(&self) -> VotingOutcome {
        // Rule 1: StrongNo from any agent = veto (safety first)
        if self.has_strong_no() {
            return VotingOutcome::Vetoed;
        }
        
        let yes_count = self.votes.values()
            .filter(|v| v.position == VotePosition::Yes || v.position == VotePosition::StrongYes)
            .count();
        let no_count = self.votes.values()
            .filter(|v| v.position == VotePosition::No || v.position == VotePosition::StrongNo)
            .count();
        
        // Rule 2: Affirmative Majority (>= 3 Yes AND Yes > No)
        if yes_count >= 3 && yes_count > no_count {
            return VotingOutcome::Approved(0.0); // Placeholder for aggregated size
        }
        
        // Rule 3: Split decision or lack of majority = highest confidence wins
        if yes_count > 0 || no_count > 0 {
            return VotingOutcome::DeferToHighestConfidence;
        }
        
        // Rule 4: Tie/Inactivity = defer to Pippo-Omega (meta-learner)
        VotingOutcome::DeferToOmega
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
