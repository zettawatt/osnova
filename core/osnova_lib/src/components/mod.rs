//! # Application Components
//!
//! Download and manage application components (frontend and backend).

pub mod downloader;

pub use downloader::{download_component, ComponentDownloader};
