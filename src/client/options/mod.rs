/// Configuration for the `Client`.
///
/// Your riot development api key can be found [here](https://developer.riotgames.com/), alternatively use your production api key.
#[derive(Clone)]
pub struct ClientOptions {
    /// Riot api key.
    pub api_key: &'static str,
    /// Setting this option will automatically retry the request if a 429 status code (too many requests) is returned from the riot api.
    pub retry: bool,
}

impl ClientOptions {
    /// Constructs an instance of `ClientOptions`.
    pub fn new(api_key: &'static str, retry: bool) -> Self {
        ClientOptions { api_key, retry }
    }
}
