//! Bridge module for in-process Rust-Python communication via PyO3.
//! 
//! This module handles the initialization of the Python interpreter 
//! and provides a unified interface for calling Python-based RL, 
//! analytics, and sentiment modules.

use pyo3::prelude::*;
use anyhow::Result;
use std::sync::Once;

pub mod rl_bridge;
pub mod analytics_bridge;
pub mod sentiment_bridge;

static START: Once = Once::new();

/// Initializes the Python interpreter and sets up the module paths.
/// 
/// This ensures that the Pippo Python modules in the `python/` directory 
/// are available to the Rust process.
pub fn init_python() {
    START.call_once(|| {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let sys = py.import("sys").unwrap();
            let path: &pyo3::types::PyList = sys.getattr("path").unwrap().downcast().unwrap();
            
            // Add the local 'python' directory to sys.path
            path.insert(0, "./python").unwrap();
        });
    });
}
