//! # FastAPI SDK
//!
//! The official Rust SDK for the FastAPI.
//!
//! ## Getting Started
//!
//! ```rust
//! use camb_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<value>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = ApiClient::new(config).expect("Failed to build client");
//!     client
//!         .audio_separation
//!         .create_audio_separation(
//!             &CreateAudioSeparationRequest {
//!                 media_file: b"test file content".to_vec(),
//!                 project_name: None,
//!                 project_description: None,
//!                 folder_id: None,
//!                 run_id: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;
pub mod provider;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::ApiError;
pub use provider::*;
