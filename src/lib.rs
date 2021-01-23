mod client;
pub mod dto;
pub mod enums;
mod error;
mod league;
mod r#match;
mod proxy;
mod summoner;

pub use client::{Client, ClientOptions};
pub use dto::*;
pub use error::Error;
