#![forbid(unsafe_code)]

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::similar_names)]

#![warn(clippy::cargo)]

mod config;

pub(crate) type AnyError = std::boxed::Box<dyn std::error::Error>;

pub type JObject = serde_json::Map<String, serde_json::Value>;

pub use config::*;

mod tests;