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
//! use osnova_lib::network::{AutonomiClient, upload_data};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = AutonomiClient::connect().await?;
//!     let data = b"Hello, Autonomi!";
//!     let address = upload_data(&client, data).await?;
//!     println!("Uploaded to: {}", address);
//!     Ok(())
//! }
//! ```

pub mod autonomi_client;
pub mod download;
pub mod upload;

pub use autonomi_client::AutonomiClient;
pub use download::download_data;
pub use upload::{estimate_upload_cost, upload_data};
