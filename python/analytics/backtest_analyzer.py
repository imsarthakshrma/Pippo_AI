# Python implementation of backtest analysis using Pandas.
import pandas as pd
import numpy as np

def run_analysis(csv_path: str, agent_id: str) -> dict:
    """
    Analyzes trade history from a CSV file and generates key metrics.
    
    Args:
        csv_path: Path to the trades CSV file.
        agent_id: ID of the agent to analyze.
        
    Returns:
        dict: Performance metrics (Sharpe, Drawdown, Win Rate).
    """
    try:
        df = pd.read_csv(csv_path)
        if df.empty:
            return {"error": "No data found"}
            
        # Filter by agent and sort chronologically
        agent_df = df[df['agent_id'] == agent_id].copy()
        if agent_df.empty:
            return {"error": f"No data for agent {agent_id}"}
            
        agent_df['timestamp'] = pd.to_datetime(agent_df['timestamp'])
        agent_df.sort_values('timestamp', inplace=True)
            
        # Calculate daily returns
        agent_df['returns'] = agent_df['profit_loss_usd'] / agent_df['initial_balance']
        
        # 1. Win Rate
        win_rate = (agent_df['profit_loss_usd'] > 0).mean()
        
        # 2. Max Drawdown
        cumulative_returns = (1 + agent_df['returns']).cumprod()
        peak = cumulative_returns.cummax()
        drawdown = (cumulative_returns - peak) / peak
        max_drawdown = drawdown.min()
        
        # 3. Annualized Sharpe Ratio
        avg_return = agent_df['returns'].mean()
        std_return = agent_df['returns'].std()
        
        if std_return != 0:
            # Empirical annualization based on trade frequency
            days_diff = (agent_df['timestamp'].max() - agent_df['timestamp'].min()).days
            if days_diff > 0:
                trades_per_year = len(agent_df) / (days_diff / 365.0)
                sharpe = (avg_return / std_return) * np.sqrt(trades_per_year)
            else:
                sharpe = (avg_return / std_return) * np.sqrt(252) # Fallback to default
        else:
            sharpe = 0.0
        
        return {
            "agent_id": agent_id,
            "win_rate": float(win_rate),
            "max_drawdown": float(max_drawdown),
            "sharpe_ratio": float(sharpe),
            "total_trades": int(len(agent_df))
        }
    except Exception as e:
        return {"error": str(e)}
