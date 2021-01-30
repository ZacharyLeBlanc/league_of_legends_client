pub mod context;
pub mod options;

use context::ClientContext;
use options::ClientOptions;
use reqwest::Client as HttpClient;

/// An asynchronous `Client` to interact with the League of Legends API.  
#[derive(Clone)]
pub struct Client {
    pub(crate) context: ClientContext,
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
