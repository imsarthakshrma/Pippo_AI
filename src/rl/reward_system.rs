use serde::{Serialize, Deserialize};

/// Quantifies the immediate reward or penalty associated with a single trade outcome.
/// 
/// This system looks beyond simple profit/loss, rewarding strategic behaviors 
/// and penalizing repeated mistakes or risk violations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeReward {
    /// Base reward for a winning trade.
    pub correct_prediction: f64,
    /// Base penalty for a losing trade.
    pub wrong_prediction: f64,
    
    /// Bonus for winning a trade where the identified edge was significant (>10%).
    pub high_edge_win: f64,
    /// Penalty for losing a trade where the agent predicted a low edge.
    pub low_edge_loss: f64,
    /// Bonus for accuracy in edge estimation.
    pub edge_validation: f64,
    
    /// Bonus for strictly following the Kelly position sizing rules.
    pub kelly_compliance: f64,
    /// Heavy penalty for exceeding internal risk limits.
    pub kelly_violation: f64,
    /// Bonus for calculating the mathematically optimal position size.
    pub position_size_optimal: f64,
    
    /// Bonus for success in a scenario that previously led to a loss.
    pub avoided_past_mistake: f64,
    /// Penalty for repeating a known losing pattern.
    pub repeated_mistake: f64,
    /// Bonus for trades that reveal new predictive correlations.
    pub new_insight: f64,
}

impl Default for TradeReward {
    fn default() -> Self {
        Self {
            correct_prediction: 100.0,
            wrong_prediction: -100.0,
            high_edge_win: 50.0,
            low_edge_loss: -200.0,
            edge_validation: 25.0,
            kelly_compliance: 10.0,
            kelly_violation: -50.0,
            position_size_optimal: 15.0,
            avoided_past_mistake: 30.0,
            repeated_mistake: -150.0,
            new_insight: 40.0,
        }
    }
}

/// Rewards agents for achieving long-term objectives and survival milestones.
pub struct MilestoneReward {
    /// Bonus for reaching $100 balance.
    pub reached_100: f64,
    /// Bonus for reaching $500 balance.
    pub reached_500: f64,
    /// Bonus for reaching $1,000 balance.
    pub reached_1000: f64,
    /// Major bonus for reaching $10,000 balance.
    pub reached_10000: f64,
    /// Reward for surviving 7 days without hitting the "Death Rule".
    pub survived_week: f64,
    /// Reward for surviving 30 days.
    pub survived_month: f64,
    /// Reward for achieving the highest return in the colony for the period.
    pub outperformed_colony: f64,
    /// Reward for reaching a personal record Sharpe ratio.
    pub best_sharpe_ratio: f64,
}

impl Default for MilestoneReward {
    fn default() -> Self {
        Self {
            reached_100: 500.0,
            reached_500: 1000.0,
            reached_1000: 2000.0,
            reached_10000: 10000.0,
            survived_week: 100.0,
            survived_month: 500.0,
            outperformed_colony: 200.0,
            best_sharpe_ratio: 150.0,
        }
    }
}

/// Penalties applied when an agent enters a state of significant risk or failure.
pub struct SurvivalPenalty {
    /// Heavy penalty when balance drops below 10% of initial stake.
    pub near_death: f64,
    /// Penalty for significant peak-to-trough decline.
    pub drawdown_warning: f64,
    /// Massive penalty for exceeding the 40% maximum drawdown limit.
    pub critical_drawdown: f64,
    /// Terminal penalty applied if the agent's balance hits zero.
    pub bankruptcy: f64,
    /// Penalty if the cost of API calls exceeds a healthy ratio relative to profit.
    pub api_cost_ratio_bad: f64,
}

impl Default for SurvivalPenalty {
    fn default() -> Self {
        Self {
            near_death: -1000.0,
            drawdown_warning: -500.0,
            critical_drawdown: -1500.0,
            bankruptcy: -10000.0,
            api_cost_ratio_bad: -100.0,
        }
    }
}
