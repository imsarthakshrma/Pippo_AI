use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeReward {
    // Outcome-based rewards
    pub correct_prediction: f64,        // Bet won
    pub wrong_prediction: f64,         // Bet lost
    
    // Strategy validation rewards
    pub high_edge_win: f64,              // >10% edge bet that won
    pub low_edge_loss: f64,            // <9% edge bet that lost
    pub edge_validation: f64,            // Actual edge matched predicted edge
    
    // Risk management rewards
    pub kelly_compliance: f64,           // Followed Kelly Criterion properly
    pub kelly_violation: f64,           // Exceeded max position size
    pub position_size_optimal: f64,      // Position size was mathematically optimal
    
    // Learning rewards
    pub avoided_past_mistake: f64,       // Didn't repeat a previous losing pattern
    pub repeated_mistake: f64,         // Made same mistake twice
    pub new_insight: f64,                // Discovered new predictive pattern
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

pub struct MilestoneReward {
    pub reached_100: f64,
    pub reached_500: f64,
    pub reached_1000: f64,
    pub reached_10000: f64,
    pub survived_week: f64,
    pub survived_month: f64,
    pub outperformed_colony: f64,
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

pub struct SurvivalPenalty {
    pub near_death: f64,
    pub drawdown_warning: f64,
    pub critical_drawdown: f64,
    pub bankruptcy: f64,
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
