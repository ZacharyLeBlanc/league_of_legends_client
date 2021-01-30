/// Configuration for the `Client`.
///
/// Your riot development api key can be found [here](https://developer.riotgames.com/), alternatively use your production api key.
#[derive(Clone)]
pub struct ClientOptions {
    /// Riot api key.
    pub(crate) api_key: String,
    /// Setting this option will automatically retry the request if a 429 status code (too many requests) is returned from the riot api.
    pub(crate) retry: bool,
}

impl ClientOptions {
    /// Constructs an instance of `ClientOptions`.
    pub fn new<T: Into<String>>(api_key: T, retry: bool) -> Self {
        ClientOptions {
            api_key: api_key.into(),
            retry,
        }
    }
}
