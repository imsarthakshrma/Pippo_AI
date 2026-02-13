//! Bridge module for in-process Rust-Python communication via PyO3.
//! 
//! This module handles the initialization of the Python interpreter 
//! and provides a unified interface for calling Python-based RL, 
//! analytics, and sentiment modules.

use pyo3::prelude::*;
use anyhow::Result;
use std::sync::OnceLock;
use std::env;

static INITIALIZED: OnceLock<Result<(), String>> = OnceLock::new();

pub mod rl_bridge;
pub mod analytics_bridge;
pub mod sentiment_bridge;

/// Initializes the Python interpreter and sets up the module paths.
pub fn init_python() -> Result<()> {
    INITIALIZED.get_or_init(|| {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let sys = py.import("sys")
                .map_err(|e| format!("Failed to import sys: {}", e))?;
            let path: &pyo3::types::PyList = sys.getattr("path")
                .map_err(|e| format!("Failed to get sys.path: {}", e))?
                .downcast()
                .map_err(|e| format!("sys.path is not a PyList: {}", e))?;
            
            // Resolve absolute path to 'python' directory relative to executable
            let mut python_path = env::current_exe()
                .map_err(|e| format!("Failed to get current_exe: {}", e))?
                .parent()
                .ok_or_else(|| "Failed to get exe parent directory".to_string())?
                .to_path_buf();
            
            // If running from target/debug, go up twice to reach project root
            if python_path.ends_with("debug") || python_path.ends_with("release") {
                python_path.pop();
                python_path.pop();
            }
            
            python_path.push("python");
            
            let path_str = python_path.to_str()
                .ok_or_else(|| "Python path contains invalid UTF-8".to_string())?;
            
            path.insert(0, path_str)
                .map_err(|e| format!("Failed to insert into sys.path: {}", e))?;
            
            Ok(())
        })
    }).clone().map_err(|e| anyhow::anyhow!(e))
}
