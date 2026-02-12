//! Market scanning and data ingestion.
//! 
//! This module defines the `Market` entity and the `MarketScanner` service 
//! for fetching active opportunities from external prediction platforms.

pub mod scanner;

pub use scanner::Market;
pub use scanner::MarketScanner;
