//! Persistence layer for the Pippo colony.
//! 
//! this module handles SQLite connectivity, database migrations, 
//! and provides the schema for trades, snapshots, and reasoning logs.

pub mod manager;
pub mod schema;

pub use manager::DbManager;
