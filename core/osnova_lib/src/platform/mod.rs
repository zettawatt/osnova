//! # Platform-Specific Utilities
//!
//! Cross-platform utilities for file paths and system integration.

pub mod paths;

pub use paths::{get_cache_dir, get_component_cache_dir, get_config_dir, get_data_dir};
