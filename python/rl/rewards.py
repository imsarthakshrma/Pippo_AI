# Python implementation of the RL reward calculation for Pippo.

def calculate_trade_reward(
    trade_outcome: str, 
    edge: float, 
    is_kelly_compliant: bool,
    avoided_mistake: bool = False
) -> float:
    """
    Quantifies the reward or penalty for a single trade.
    
    Args:
        trade_outcome: "Win" or "Loss"
        edge: The identified edge percentage (e.g., 0.12 for 12%)
        is_kelly_compliant: Whether the position size followed Kelly limits
        avoided_mistake: Bonus for learning from past failures
        
    Returns:
        float: The calculated reward value.
    """
    reward = 0.0
    
    # 1. Outcome-based rewards
    if trade_outcome == "Win":
        reward += 100.0
        # Strategic bonus for high-edge wins
        if edge >= 0.10:
            reward += 50.0
    else:
        reward -= 100.0
        # Heavy penalty for losing on low-edge bets
        if edge < 0.09:
            reward -= 200.0
            
    # 2. Risk management rewards
    if is_kelly_compliant:
        reward += 10.0
    else:
        reward -= 50.0 # Heavy penalty for risk violations
        
    # 3. Learning rewards
    if avoided_mistake:
        reward += 30.0
        
    return float(reward)

def calculate_milestone_reward(milestone_type: str) -> float:
    """Returns the fixed reward for achieving specific milestones."""
    milestones = {
        "reached_100": 500.0,
        "reached_1000": 2000.0,
        "survived_week": 100.0,
        "survived_month": 500.0
    }
    return float(milestones.get(milestone_type, 0.0))
