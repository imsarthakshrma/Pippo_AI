use tracing::{error, info};
use std::collections::HashMap;

/// Manages the core financial state and survival rules for the colony.
/// 
/// The `RlSystem` tracks agent balances and enforces the "Death Rule," 
/// ensuring the colony shuts down if capital is exhausted.
pub struct RlSystem {
    /// Mapping of agent names to their current allocated USD balance.
    pub agent_balances: HashMap<String, f64>,
    /// The initial total capital split among the colony.
    pub total_initial_capital: f64,
}

impl RlSystem {
    /// Initializes the `RlSystem` by splitting the total capital among the starting agents.
    pub fn new(total_capital: f64, agent_names: Vec<String>) -> Self {
        let mut agent_balances = HashMap::new();
        let split_capital = total_capital / agent_names.len() as f64;
        
        for name in agent_names {
            agent_balances.insert(name, split_capital);
        }

        Self {
            agent_balances,
            total_initial_capital: total_capital,
        }
    }

    /// Checks if any agents have hit a $0 balance and removes them from the colony.
    /// 
    /// If the last agent fails, it returns `true` to indicate a complete colony collapse.
    pub fn check_death_rule(&mut self) -> bool {
        let mut failed_agents = Vec::new();

        for (name, balance) in &self.agent_balances {
            if *balance <= 0.0 {
                failed_agents.push(name.clone());
            }
        }

        if failed_agents.is_empty() {
            return false;
        }

        for name in failed_agents {
            error!("Agent {} has hit $0 and has shut down permanently.", name);
            self.agent_balances.remove(&name);
        }

        if self.agent_balances.is_empty() {
            error!("The entire colony has collapsed. Permanent shutdown initiated.");
            return true;
        }

        info!("Survivors remain. Colony continues.");
        false
    }

    /// Updates an agent's balance based on trade profit/loss and records achievements.
    pub fn process_milestones(&mut self, agent_id: &str, profit: f64) {
        if let Some(balance) = self.agent_balances.get_mut(agent_id) {
            *balance += profit;
            info!("Agent {} balance updated: ${}", agent_id, *balance);
            
            // TODO: Persist specific milestones ($100, $500, etc.) to DB
        }
    }
}
