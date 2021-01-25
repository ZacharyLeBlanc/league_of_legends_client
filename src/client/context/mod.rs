use super::options::ClientOptions;
use reqwest::Client as HttpClient;

/// Context that is passed around for each request.
#[derive(Clone)]
pub struct ClientContext {
    /// The `reqwest::Client` used for each request.
    pub http_client: HttpClient,
    /// The `ClientOptions` for this `Client`.
    pub options: ClientOptions,
}
