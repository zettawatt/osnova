//! # Network Module
//!
//! Network operations for Autonomi Network integration.
//!
//! This module provides:
//! - Autonomi client connection management
//! - Data upload and download operations
//! - Component caching and retrieval
//!
//! ## Example
//!
//! ```rust,ignore
//! use osnova_lib::network::AutonomiClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = AutonomiClient::connect().await?;
//!     let is_healthy = client.health_check().await?;
//!     println!("Network healthy: {}", is_healthy);
//!     Ok(())
//! }
//! ```

pub mod autonomi_client;

pub use autonomi_client::AutonomiClient;
