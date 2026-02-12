//! Trading logic and risk management core.
//! 
//! This module contains the `MarketAnalyzer` for probability estimation, 
//! the `KellySizer` for position sizing, and the `TradeExecutor` for 
//! platform interactions.

pub mod kelly;
pub mod executor;
pub mod analyzer;

pub use kelly::KellySizer;
pub use executor::TradeExecutor;
pub use analyzer::MarketAnalyzer;
