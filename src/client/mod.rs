use reqwest::Client as HttpClient;

#[derive(Copy, Clone)]
pub struct ClientOptions {
    pub api_key: &'static str,
    pub retry: bool,
}

impl ClientOptions {
    pub fn new(api_key: &'static str, retry: bool) -> Self {
        ClientOptions { api_key, retry }
    }
}

#[derive(Clone)]
pub struct ClientContext {
    pub http_client: HttpClient,
    pub options: ClientOptions,
}

#[derive(Clone)]
pub struct Client {
    pub context: ClientContext,
}

impl Client {
    pub fn new(options: ClientOptions) -> Self {
        Client {
            context: ClientContext {
                options,
                http_client: HttpClient::new(),
            },
        }
    }
}
