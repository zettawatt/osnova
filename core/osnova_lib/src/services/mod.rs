//! Core services for Osnova
//!
//! This module provides the OpenRPC-based services:
//! - Identity management
//! - Key derivation and management
//! - Configuration management
//! - Application management
//! - Storage operations

/// Identity management service
pub mod identity;

/// Key derivation and management service
pub mod keys;

/// Configuration management service
pub mod config;

/// Application management service
pub mod apps;

/// Launcher layout service
pub mod launcher;

/// UI management service
pub mod ui;

/// Navigation management service
pub mod navigation;

/// Status management service
pub mod status;

pub use identity::IdentityService;
pub use keys::KeyService;
pub use config::ConfigService;
pub use apps::AppsService;
pub use launcher::LauncherService;
pub use ui::{UIService, Theme};
pub use navigation::{NavigationService, BottomMenuTab};
pub use status::{StatusService, ServerStatus, ServerStatusResponse};
