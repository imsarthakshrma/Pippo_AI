mod claude;
mod market;
mod db;
mod trading;
mod config;
mod logic;
mod monitoring;
mod colony;
mod blog;
mod backtesting;

use anyhow::Result;
use tokio::time::{self, Duration};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Pippo is waking up...");

    // 1. Load configuration
    let cfg = crate::config::load()?;
    
    // 2. Initialize components
    let colony_manager = colony::colony_manager::ColonyManager::new();
    let agent_names: Vec<String> = colony_manager.agents().iter().map(|a| a.name().to_string()).collect();
    
    let analyzer = trading::analyzer::MarketAnalyzer::new(cfg.claude.api_key.clone());
    let mut rl = logic::RlSystem::new(cfg.trading.balance_usd, agent_names);

    // Core loop
    let mut interval = time::interval(Duration::from_secs(cfg.trading.interval_minutes * 60));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if rl.check_death_rule() {
                    break;
                }

                let total_balance: f64 = rl.agent_balances.values().sum();
                info!("Starting new trading cycle. Total colony balance: ${}", total_balance);

                if let Err(e) = run_cycle(&analyzer, &cfg).await {
                    error!("Error in trading cycle: {:?}", e);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Shutdown signal received. Cleaning up...");
                break;
            }
        }
    }

    info!("Pippo has shut down.");
    Ok(())
}

async fn run_cycle(analyzer: &trading::analyzer::MarketAnalyzer, cfg: &crate::config::PippoConfig) -> Result<()> {
    info!("Scanning markets...");
    
    // 1. Fetch market data
    let markets = market::MarketScanner::scan_active_markets().await?;
    if markets.is_empty() {
        info!("No active markets found.");
        return Ok(());
    }

    // 2. Analyze markets via Claude
    let budget = cfg.claude.thinking_budget_default;
    let analysis_results = analyzer.analyze_markets(&markets, budget).await?;

    for res in analysis_results {
        if res.edge >= cfg.trading.min_edge {
            info!("Identify opportunity: {} (Edge: {:.2}%)", res.market_id, res.edge * 100.0);
            
            // 3. Calculate position size
            if let Some(market) = markets.iter().find(|m| m.id == res.market_id) {
                let odds = market.current_odds[0]; 
                let size = trading::KellySizer::calculate_position_size(
                    rl_balance_demo(cfg.trading.balance_usd), // Demo placeholder
                    odds, 
                    res.fair_value, 
                    cfg.trading.max_kelly_fraction
                );

                if size > 0.0 {
                    info!("Sizing position: ${:.2} on {}", size, res.market_id);
                    // 4. Execute trade
                    trading::executor::TradeExecutor::execute_trade(res.market_id, size).await?;
                }
            }
        }
    }

    Ok(())
}

fn rl_balance_demo(val: f64) -> f64 { val }
