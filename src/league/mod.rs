use super::{
    enums::{Division, Queue, Region, Tier},
    proxy::{request, Result},
    types::{LeagueEntry, LeagueList},
    Client, ClientContext,
};
use url::Url;

impl Client {
    /// Access the league namespace that contains league related API methods.
    ///
    /// [Riot API documentation](https://developer.riotgames.com/apis#league-v4).
    ///
    /// Use this method instead of manually constructing a `League` struct.
    /// # Example
    ///
    /// ```rust
    /// # use league_of_legends_client::{Client, ClientOptions};
    /// let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    ///
    /// // Access the league API methods.
    /// let league = client.league();
    /// ```
    pub fn league(&self) -> League {
        League {
            context: &self.context,
        }
    }
}

/// Namespace for the league related APIs.
///
/// [Riot API documentation](https://developer.riotgames.com/apis#league-v4).
///
/// Do not create this struct manually. The preferred method is to call `client.league()`.
///
/// # Example
///
/// ```rust
/// # use league_of_legends_client::{
/// #    enums::{Queue, Region},
/// #    Client, ClientOptions,
/// #    Error,
/// # };
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), Error> {
/// let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
///
/// // Access the league API methods.
/// let mut challenger_league = client
///     .league()
///     .get_challenger_league(Region::NorthAmerica, Queue::RankedSoloQueue)
///     .await;
///
/// // Alternatively use this to save some characters.
/// let league = client.league();
///
/// challenger_league = league
///     .get_challenger_league(Region::NorthAmerica, Queue::RankedSoloQueue)
///     .await;
/// let grandmaster_league = league
///     .get_grandmaster_league(Region::NorthAmerica, Queue::RankedSoloQueue)
///     .await;
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct League<'a> {
    context: &'a ClientContext,
}

impl League<'_> {
    /// Get the challenger league for given queue.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#league-v4/GET_getChallengerLeague).
    ///
    /// # Example
    /// ```rust
    /// # use league_of_legends_client::{
    /// #    enums::{Queue, Region},
    /// #    Client, ClientOptions,
    /// #    Error,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    /// let challenger_league = client
    ///     .league()
    ///     .get_challenger_league(Region::NorthAmerica, Queue::RankedSoloQueue)
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_challenger_league(&self, region: Region, queue: Queue) -> Result<LeagueList> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/challengerleagues/by-queue/{}",
            region.to_string(),
            queue.to_string()
        ))?;

        request::<LeagueList>(url.as_str(), self.context).await
    }

    /// Get league entries in all queues for a given summoner ID.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#league-v4/GET_getLeagueEntriesForSummoner).
    ///
    /// # Example
    /// ```rust
    /// # use league_of_legends_client::{
    /// #    enums::Region,
    /// #    Client, ClientOptions,
    /// #    Error,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    /// let challenger_league = client
    ///     .league()
    ///     .get_league_entries_for_summoner(
    ///         Region::NorthAmerica,
    ///         "EXAMPLE_ENCRYPTED_SUMMONER_ID".to_string(),
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_league_entries_for_summoner(
        &self,
        region: Region,
        encrypted_summoner_id: String,
    ) -> Result<Vec<LeagueEntry>> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/entries/by-summoner/{}",
            region.to_string(),
            encrypted_summoner_id
        ))?;

        request::<Vec<LeagueEntry>>(url.as_str(), self.context).await
    }

    /// Get all the league entries.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#league-v4/GET_getLeagueEntries).
    ///
    /// # Example
    /// ```rust
    /// # use league_of_legends_client::{
    /// #    enums::{Division, Queue, Region, Tier},
    /// #    Client, ClientOptions,
    /// #    Error,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    /// let challenger_league = client
    ///     .league()
    ///     .get_league_entries(
    ///         Region::NorthAmerica,
    ///         Queue::RankedSoloQueue,
    ///         Tier::Diamond,
    ///         Division::Four,
    ///         None
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_league_entries(
        &self,
        region: Region,
        queue: Queue,
        tier: Tier,
        division: Division,
        page: Option<i8>,
    ) -> Result<Vec<LeagueEntry>> {
        let mut url = Url::parse(&format!(
            "https://{}/lol/league/v4/entries/{}/{}/{}",
            region.to_string(),
            queue,
            tier,
            division,
        ))?;

        if let Some(page) = page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        request::<Vec<LeagueEntry>>(url.as_str(), self.context).await
    }

    /// Get the grandmaster league of a specific queue.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#league-v4/GET_getGrandmasterLeague).
    ///
    /// # Example
    /// ```rust
    /// # use league_of_legends_client::{
    /// #    enums::{Queue, Region},
    /// #    Client, ClientOptions,
    /// #    Error,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    /// let challenger_league = client
    ///     .league()
    ///     .get_grandmaster_league(Region::NorthAmerica, Queue::RankedSoloQueue)
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_grandmaster_league(&self, region: Region, queue: Queue) -> Result<LeagueList> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/grandmasterleagues/by-queue/{}",
            region.to_string(),
            queue.to_string()
        ))?;

        request::<LeagueList>(url.as_str(), self.context).await
    }

    /// Get league with given ID, including inactive entries.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#league-v4/GET_getLeagueById).
    ///
    /// **Consistently looking up league ids that don't exist will result in a blacklist.**
    ///
    /// # Example
    /// ```rust
    /// # use league_of_legends_client::{
    /// #    enums::Region,
    /// #    Client, ClientOptions,
    /// #    Error,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    /// let challenger_league = client
    ///     .league()
    ///     .get_league(Region::NorthAmerica, "EXAMPLE_LEAGUE_ID".to_string())
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_league(&self, region: Region, league_id: String) -> Result<LeagueList> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/leagues/{}",
            region.to_string(),
            league_id
        ))?;

        request::<LeagueList>(url.as_str(), self.context).await
    }

    /// Get the master league for given queue.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#league-v4/GET_getMasterLeague).
    ///
    /// # Example
    /// ```rust
    /// # use league_of_legends_client::{
    /// #    enums::{Queue, Region},
    /// #    Client, ClientOptions,
    /// #    Error,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    /// let challenger_league = client
    ///     .league()
    ///     .get_master_league(Region::NorthAmerica, Queue::RankedSoloQueue)
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_master_league(&self, region: Region, queue: Queue) -> Result<LeagueList> {
        let url = Url::parse(&format!(
            "https://{}/lol/league/v4/masterleagues/by-queue/{}",
            region.to_string(),
            queue.to_string()
        ))?;

        request::<LeagueList>(url.as_str(), self.context).await
    }
}
