use super::{
    enums::Region,
    proxy::{request, Result},
    types::{Match as MatchDTO, MatchList},
    Client, ClientContext,
};
use std::collections::HashSet;
use url::Url;

impl Client {
    /// Access the match namespace that contains match related API methods.
    ///
    /// Unfortunately in rust `match` is a keyword and needs to be escaped with `r#`, see the example below.
    ///
    /// [Riot API documentation](https://developer.riotgames.com/apis#match-v4).
    ///
    /// Use this method instead of manually constructing a `Match` struct.
    /// # Example
    ///
    /// ```rust
    /// # use league_of_legends_client::{Client, ClientOptions};
    /// let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
    ///
    /// // Access the match API methods.
    /// let match_api = client.r#match();
    /// ```
    pub fn r#match(&self) -> Match {
        Match {
            context: &self.context,
        }
    }
}

/// Namespace for the match related APIs.
///
/// Unfortunately in rust `match` is a keyword and needs to be escaped with `r#`, see the example below.
///
/// [Riot API documentation](https://developer.riotgames.com/apis#match-v4).
///
/// Do not create this struct manually. The preferred method is to call `client.r#match()`.
///
/// # Example
///
/// ```rust
/// # use league_of_legends_client::{
/// #    enums::Region,
/// #    Client, ClientOptions,
/// #    Error,
/// # };
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), Error> {
/// let client = Client::new(ClientOptions::new("EXAMPLE_API_KEY", false));
///
/// // Access the match API methods.
/// let mut game = client
///     .r#match()
///     .get_match_by_id(Region::NorthAmerica, 00000)
///     .await;
///
/// // Alternatively use this to save some characters.
/// let match_api = client.r#match();
///
/// game = match_api
///     .get_match_by_id(Region::NorthAmerica, 00000)
///     .await;
/// let match_list = match_api
///     .get_match_list_by_account(
///         Region::NorthAmerica,
///         "EXAMPLE_ENCRYPTED_ACCOUNT_ID".to_string(),
///         None,
///         None,
///         None,
///         None,
///         None,
///         None,
///      )
///     .await;
/// #
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Match<'a> {
    context: &'a ClientContext,
}

impl Match<'_> {
    /// Get match by match ID.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#match-v4/GET_getMatch).
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
    /// let game = client
    ///     .r#match()
    ///     .get_match_by_id(Region::NorthAmerica, 00000)
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_match_by_id(&self, region: Region, match_id: i64) -> Result<MatchDTO> {
        let url = Url::parse(&format!(
            "https://{}/lol/match/v4/matches/{}",
            region.to_string(),
            match_id.to_string()
        ))?;

        request::<MatchDTO>(url.as_str(), self.context).await
    }

    /// Get matchlist for games played on given account ID and platform ID and filtered using given filter parameters, if any.
    ///
    /// API documentation can be found [here](https://developer.riotgames.com/apis#match-v4/GET_getMatchlist).
    ///
    /// ## IMPLEMENTATION NOTES
    ///
    /// A number of optional parameters are provided for filtering. It is up to the caller to ensure that the
    /// combination of filter parameters provided is valid for the requested account, otherwise, no matches may be
    /// returned. If beginIndex is specified, but not endIndex, then endIndex defaults to beginIndex+100. If endIndex
    /// is specified, but not beginIndex, then beginIndex defaults to 0. If both are specified, then endIndex must be
    /// greater than beginIndex. The maximum range allowed is 100, otherwise a 400 error code is returned. If
    /// beginTime is specified, but not endTime, then endTime defaults to the the current unix timestamp in
    /// milliseconds (the maximum time range limitation is not observed in this specific case). If endTime is
    /// specified, but not beginTime, then beginTime defaults to the start of the account's match history returning a
    /// 400 due to the maximum time range limitation. If both are specified, then endTime should be greater than
    /// beginTime. The maximum time range allowed is one week, otherwise a 400 error code is returned.
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
    /// let game = client
    ///     .r#match()
    ///     .get_match_list_by_account(
    ///         Region::NorthAmerica,
    ///         "EXAMPLE_ENCRYPTED_ACCOUNT_ID".to_string(),
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///     )
    ///     .await;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_match_list_by_account(
        &self,
        region: Region,
        encrypted_account_id: String,
        champion: Option<HashSet<i32>>,
        queue: Option<HashSet<i32>>,
        end_time: Option<i64>,
        begin_time: Option<i64>,
        end_index: Option<i32>,
        begin_index: Option<i32>,
    ) -> Result<MatchList> {
        let mut url = Url::parse(&format!(
            "https://{}/lol/match/v4/matchlists/by-account/{}",
            region.to_string(),
            encrypted_account_id
        ))?;

        if let Some(champion) = champion {
            for champion in champion {
                url.query_pairs_mut()
                    .append_pair("champion", &champion.to_string());
            }
        }

        if let Some(queue) = queue {
            for queue in queue {
                url.query_pairs_mut()
                    .append_pair("queue", &queue.to_string());
            }
        }

        if let Some(end_time) = end_time {
            url.query_pairs_mut()
                .append_pair("endTime", &end_time.to_string());
        }

        if let Some(begin_time) = begin_time {
            url.query_pairs_mut()
                .append_pair("beginTime", &begin_time.to_string());
        }

        if let Some(end_index) = end_index {
            url.query_pairs_mut()
                .append_pair("endIndex", &end_index.to_string());
        }

        if let Some(begin_index) = begin_index {
            url.query_pairs_mut()
                .append_pair("begin_index", &begin_index.to_string());
        }

        request::<MatchList>(url.as_str(), self.context).await
    }
}
