#![deny(clippy::unwrap_used)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/SteelAlloy/thumbor-rs/main/assets/doc/logo.svg"
)]

//! # Thumbor
//!
//! > open-source smart on-demand image cropping, resizing and filters
//!
//! This library is a Rust client implementation of the Thumbor image service
//! to generate image urls.
//!
//! # Usage
//!
//! ```
//! use thumbor::Server;
//!
//! let server = Server::new("http://localhost:8888", "my-security-key").unwrap();
//!
//! let endpoint = server.endpoint_builder()
//!     .resize((300, 200))
//!     .smart(true)
//!     .build();
//!
//! let url = endpoint.to_url("path/to/my/image.jpg");
//! ```

pub mod endpoint;
pub mod error;
pub mod geometry;
mod server;

#[cfg(test)]
mod tests;

pub use endpoint::{filter::Filter, Endpoint, EndpointBuilder};
pub use server::Server;
