//! Observability and logging infrastructure.
//! 
//! This module configures the `tracing` subscriber to capture and filter 
//! logs from all Pippo components.

use tracing::{Level, Subscriber};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;

/// Initializes the global tracing subscriber for logging.
/// 
/// This sets up a `FmtSubscriber` that outputs logs to stdout.
/// It defaults to `Level::INFO` but can be extended to support configuration-based levels.
pub fn init_telemetry() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
