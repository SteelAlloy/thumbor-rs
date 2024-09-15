#![deny(clippy::unwrap_used)]
#![doc(html_logo_url = "/logo.svg")]

pub mod error;
pub mod geometry;
mod server;
pub mod settings;

#[cfg(test)]
mod tests;

pub use server::Server;
pub use settings::{filter::Filter, SettingsBuilder};
