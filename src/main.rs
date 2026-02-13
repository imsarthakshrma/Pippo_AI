//! Entry point for the Pippo trading agent.
//! 
//! This binary orchestrates the full lifecycle of the trading colony, including:
//! - Configuration loading
//! - Component initialization (Colony, Analyzer, RL System)
//! - The main trading loop (market scanning, analysis, execution)
//! - Interactive CLI chat with Pippo personalities (`--chat`)
//! - Graceful shutdown handling

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
mod cli;

use anyhow::Result;
use tokio::time::{self, Duration};
use tracing::{error, info};
use crate::trading::analyzer::MarketAnalyzerTrait;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (for API keys and secrets)
    dotenv::dotenv().ok();

    // Initialize logging via the monitoring module
    monitoring::init_telemetry()?;

    info!("Pippo is waking up...");

    // 1. Load configuration
    let cfg = crate::config::load()?;
    
    // Check for --chat flag
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--chat") {
        return run_chat_mode(&cfg).await;
    }

    // 2. Initialize database
    let db_manager = db::DbManager::new(&cfg.trading.database_url).await?;
    
    // 3. Initialize components
    let colony_manager = colony::colony_manager::ColonyManager::new(db_manager.pool().clone());
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

/// Launches the interactive Pippo Chat REPL.
async fn run_chat_mode(cfg: &crate::config::PippoConfig) -> Result<()> {
    use cli::personality::{ColonyContext, PippoPersonality};
    use cli::repl::{PippoChat, run_repl};
    use std::collections::HashMap;

    // Build colony context (from DB if available, otherwise from config defaults)
    let context = match db::DbManager::new(&cfg.trading.database_url).await {
        Ok(db) => {
            let colony = colony::colony_manager::ColonyManager::new(db.pool().clone());
            let agent_names: Vec<String> = colony.agents().iter().map(|a| a.name().to_string()).collect();
            let rl = logic::RlSystem::new(cfg.trading.balance_usd, agent_names.clone());
            
            ColonyContext {
                balances: rl.agent_balances.clone(),
                recent_trades: Vec::new(), // TODO: query recent trades from DB
                initial_capital: rl.total_initial_capital,
            }
        }
        Err(_) => {
            // Fallback: use config defaults with no trade history
            let mut balances = HashMap::new();
            let per_agent = cfg.trading.balance_usd / 5.0;
            for name in &["Pippo-Alpha", "Pippo-Beta", "Pippo-Gamma", "Pippo-Delta", "Pippo-Omega"] {
                balances.insert(name.to_string(), per_agent);
            }
            ColonyContext {
                balances,
                recent_trades: Vec::new(),
                initial_capital: cfg.trading.balance_usd,
            }
        }
    };

    let client = claude::client::ClaudeClient::new(cfg.claude.api_key.clone());
    let mut chat = PippoChat::new(
        client,
        context,
        cfg.claude.model.clone(),
        512, // Short responses for personality chat
    );

    run_repl(&mut chat).await
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
