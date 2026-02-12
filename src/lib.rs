//! # Pippo: Autonomous Multi-Agent Trading Colony
//! 
//! Pippo is a decentralized multi-agent system designed for prediction market trading.
//! It features:
//! - **The Colony**: Five distinct agent personalities (Alpha, Beta, Gamma, Delta, Omega).
//! - **Backtesting**: Comprehensive historical data collection and simulation.
//! - **Narrative**: Personality-driven experiential blogs based on market activities.
//! - **Intelligence**: Powered by Claude's Extended Thinking budget for deep analysis.

pub mod config;
pub mod claude;
pub mod db;
pub mod market;
pub mod trading;
pub mod logic;
pub mod monitoring;
pub mod colony;
pub mod blog;
pub mod backtesting;
pub mod rl;
