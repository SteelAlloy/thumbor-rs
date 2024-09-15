#![deny(clippy::unwrap_used)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/SteelAlloy/thumbor-rs/main/assets/doc/logo.svg"
)]

pub mod endpoint;
pub mod error;
pub mod geometry;
mod server;

#[cfg(test)]
mod tests;

pub use endpoint::{filter::Filter, Endpoint, EndpointBuilder};
pub use server::Server;
