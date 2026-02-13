//! Backtesting infrastructure for the Pippo trading colony.
//! 
//! This module provides tools for historical data collection, market simulation, 
//! performance metrics, and offline RL training.

pub mod data_sources;
pub mod data_collector;
pub mod simulator;
pub mod mock_analyzer;
pub mod metrics;
pub mod rl_trainer;
