//! Storage backends for Osnova
//!
//! This module provides storage implementations:
//! - SQLite storage for structured data
//! - File-based encrypted storage for cache and keys
//! - Encrypted blob storage

/// SQLite storage backend
pub mod sql;

/// File-based encrypted storage
pub mod file;

pub use file::FileStorage;
pub use sql::SqlStorage;
