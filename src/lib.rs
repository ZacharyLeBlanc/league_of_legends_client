//! This crate is an asynchronous client to interact with the League of Legends API.
//!
//! ![CI](https://github.com/ZacharyLeBlanc/league_of_legends_client/workflows/CI/badge.svg)
//!
//! The Riot API development portal can be found [here](https://developer.riotgames.com/). Please read through the
//! [documentation](https://developer.riotgames.com/docs/portal) and
//! [policies](https://developer.riotgames.com/policies/general) before getting started. Riot API endpoints can be
//! found [here](https://developer.riotgames.com/apis).
//!
//! \* *Disclaimer: There certain conditions where your API key can be revoked and you can be blacklisted from their
//! API. Please be responsible and read the documentation, policies, and rules. The creator and maintainers of this
//! project will not accept any responsibility for you getting blacklisted.*
//!
//! As of right now this crate uses only tokio as its runtime.
//!
//! # Examples
//!
//! ```toml
//! league_of_legends_client = "0.1"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! ```rust
//! use league_of_legends_client::{
//!    enums::{Queue, Region},
//!    Client, ClientOptions,
//!    Error,
//!};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
//!
//!     // Get list of challenger league entries for solo queue in North America.
//!     let challenger_league_entries_request = client
//!         .league()
//!         .get_challenger_league(Region::NorthAmerica, Queue::RankedSoloQueue)
//!         .await;
//!
//!     match challenger_league_entries_request {
//!         Ok(challenger_league_entries) => println!("{:#?}", challenger_league_entries),
//!         Err(error) => println!("Oh no! An error occurred! Error: {:#?}", error),
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
pub mod enums;
mod error;
mod league;
mod r#match;
mod proxy;
mod summoner;
pub mod types;

use client::context::ClientContext;
pub use client::options::ClientOptions;
pub use error::Error;
pub use league::League;
pub use r#match::Match;
use reqwest::Client as HttpClient;
pub use summoner::Summoner;

/// An asynchronous `Client` to interact with the League of Legends API.  
#[derive(Clone)]
pub struct Client {
    context: ClientContext,
}

impl Client {
    /// Creates a new instance of `Client`.
    pub fn new(options: ClientOptions) -> Self {
        Client {
            context: ClientContext {
                options,
                http_client: HttpClient::new(),
            },
        }
    }
}
