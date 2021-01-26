use super::{
    enums::Region,
    proxy::{request, Result},
    types::Summoner as SummonerDTO,
    Client, ClientContext,
};
use url::Url;

impl Client {
    /// Access the summoner namespace that contains league related API methods.
    ///
    /// [Riot API documentation](https://developer.riotgames.com/apis#summoner-v4).
    ///
    /// Use this method instead of manually constructing a `Summoner` struct.
    /// # Example
    ///
    /// ```rust
    /// # use league_of_legends_client::{Client, ClientOptions};
    /// let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    ///
    /// // Access the summoner API methods.
    /// let summoner_api = client.league();
    /// ```
    pub fn summoner(&self) -> Summoner {
        Summoner {
            context: &self.context,
        }
    }
}

/// Namespace for the summoner related APIs.
///
/// [Riot API documentation](https://developer.riotgames.com/apis#summoner-v4).
///
/// Do not create this struct manually. The preferred method is to call `client.summoner()`.
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
/// // Access the summoner API methods.
/// let mut summoner_one = client
///     .summoner()
///     .get_summoner_by_encrypted_account_id(
///         Region::NorthAmerica,
///         "EXAMPLE_ENCRYPTED_ACCOUNT_ID".to_string()
///     )
///     .await;
///
/// // Alternatively use this to save some characters.
/// let summoner_api = client.summoner();
///
/// summoner_one = summoner_api
///     .get_summoner_by_encrypted_account_id(
///         Region::NorthAmerica,
///         "EXAMPLE_ENCRYPTED_ACCOUNT_ID".to_string()
///     )
///     .await;
/// let summoner_two = summoner_api
///     .get_summoner_by_summoner_name(
///         Region::NorthAmerica,
///         "EXAMPLE_NAME".to_string()
///     )
///     .await;
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Summoner<'a> {
    context: &'a ClientContext,
}

impl Summoner<'_> {
    /// Get a summoner by account ID.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#summoner-v4/GET_getByAccountId).
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
    /// let summoner = client
    ///     .summoner()
    ///     .get_summoner_by_encrypted_account_id(
    ///         Region::NorthAmerica,
    ///         "EXAMPLE_ENCRYPTED_ACCOUNT_ID".to_string()
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_summoner_by_encrypted_account_id(
        &self,
        region: Region,
        encrypted_account_id: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/by-account/{}",
            region.to_string(),
            encrypted_account_id
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }

    /// Get a summoner by summoner name.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#summoner-v4/GET_getBySummonerName).
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
    /// let summoner = client
    ///     .summoner()
    ///     .get_summoner_by_summoner_name(
    ///         Region::NorthAmerica,
    ///         "EXAMPLE_SUMMONER_NAME".to_string()
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_summoner_by_summoner_name(
        &self,
        region: Region,
        summoner_name: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/by-name/{}",
            region.to_string(),
            summoner_name
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }

    /// Get a summoner by PUUID.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#summoner-v4/GET_getByPUUID).
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
    /// let summoner = client
    ///     .summoner()
    ///     .get_summoner_by_encrypted_puuid(
    ///         Region::NorthAmerica,
    ///         "EXAMPLE_PUUID".to_string()
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_summoner_by_encrypted_puuid(
        &self,
        region: Region,
        encrypted_puu_id: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/by-puuid/{}",
            region.to_string(),
            encrypted_puu_id
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }

    /// Get a summoner by summoner ID.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#summoner-v4/GET_getBySummonerId).
    ///
    /// **Consistently looking up summoner ids that don't exist will result in a blacklist.**
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
    /// let summoner = client
    ///     .summoner()
    ///     .get_summoner_by_encrypted_summoner_id(
    ///         Region::NorthAmerica,
    ///         "EXAMPLE_ENCRYPTED_SUMMONER_ID".to_string()
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_summoner_by_encrypted_summoner_id(
        &self,
        region: Region,
        encrypted_summoner_id: String,
    ) -> Result<SummonerDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/summoner/v4/summoners/{}",
            region.to_string(),
            encrypted_summoner_id
        ))?;

        request::<SummonerDTO>(url.as_str(), self.context).await
    }
}
