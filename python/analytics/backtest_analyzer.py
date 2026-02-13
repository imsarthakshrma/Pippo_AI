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
            
        # Filter by agent
        agent_df = df[df['agent_id'] == agent_id].copy()
        if agent_df.empty:
            return {"error": f"No data for agent {agent_id}"}
            
        # Calculate daily returns
        agent_df['returns'] = agent_df['profit_loss_usd'] / agent_df['initial_balance']
        
        # 1. Win Rate
        win_rate = (agent_df['profit_loss_usd'] > 0).mean()
        
        # 2. Max Drawdown
        cumulative_returns = (1 + agent_df['returns']).cumprod()
        peak = cumulative_returns.cummax()
        drawdown = (cumulative_returns - peak) / peak
        max_drawdown = drawdown.min()
        
        # 3. Sharpe Ratio (assumed daily)
        avg_return = agent_df['returns'].mean()
        std_return = agent_df['returns'].std()
        sharpe = (avg_return / std_return) * np.sqrt(252) if std_return != 0 else 0
        
        return {
            "agent_id": agent_id,
            "win_rate": float(win_rate),
            "max_drawdown": float(max_drawdown),
            "sharpe_ratio": float(sharpe),
            "total_trades": int(len(agent_df))
        }
    except Exception as e:
        return {"error": str(e)}
