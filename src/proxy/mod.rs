mod rate_limit;

use crate::{ClientContext, Error};
use log::{debug, error};
use rate_limit::RateLimit;
use reqwest::{header::HeaderMap, Response, StatusCode};
use serde::de::DeserializeOwned;

pub type Result<T> = std::result::Result<T, Error>;

fn parse_headers(header_map: &HeaderMap) -> RateLimit {
    debug!("Response headers: {:?}", header_map);
    let rate_limit = RateLimit::from(header_map);
    debug!("{}", rate_limit);
    rate_limit
}

async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T> {
    let rate_limit = parse_headers(response.headers());

    match response.error_for_status() {
        Ok(response) => Ok(response.json::<T>().await?),
        Err(error) => {
            error!("API came back with error response: {}", error);
            match error.status() {
                Some(status_code) => match status_code {
                    StatusCode::BAD_REQUEST => {
                        debug!("400 (Bad Request) This error indicates that there is a syntax error in the request and the request has therefore been denied. The client should not continue to make similar requests without modifying the syntax or the requests being made.");
                        debug!("Common Reasons:");
                        debug!("A provided parameter is in the wrong format (e.g., a string instead of an integer).");
                        debug!("A provided parameter is invalid (e.g., beginTime and startTime specify a time range that is too large).");
                        debug!("A required parameter was not provided.");
                        Err(Error::BadRequest)
                    }
                    StatusCode::UNAUTHORIZED => {
                        debug!("401 (Unauthorized) This error indicates that the request being made did not contain the necessary authentication credentials (e.g., an API key) and therefore the client was denied access. The client should not continue to make similar requests without including an API key in the request.");
                        debug!("Common Reasons:");
                        debug!("An API key has not been included in the request.");
                        Err(Error::Unauthorized)
                    }
                    StatusCode::FORBIDDEN => {
                        debug!("403 (Forbidden) This error indicates that the server understood the request but refuses to authorize it. There is no distinction made between an invalid path or invalid authorization credentials (e.g., an API key). The client should not continue to make similar requests.");
                        debug!("Common Reasons:");
                        debug!("An invalid API key was provided with the API request.");
                        debug!("A blacklisted API key was provided with the API request.");
                        debug!("The API request was for an incorrect or unsupported path.");
                        Err(Error::Forbidden)
                    }
                    StatusCode::NOT_FOUND => {
                        debug!("404 (Not Found) This error indicates that the server has not found a match for the API request being made. No indication is given whether the condition is temporary or permanent.");
                        debug!("Common Reasons:");
                        debug!("The ID or name provided does not match any existing resource (e.g., there is no Summoner matching the specified ID).");
                        debug!("There are no resources that match the parameters specified.");
                        Err(Error::NotFound)
                    }
                    StatusCode::UNSUPPORTED_MEDIA_TYPE => {
                        debug!("415 (Unsupported Media Type) This error indicates that the server is refusing to service the request because the body of the request is in a format that is not supported.");
                        debug!("Common Reasons:");
                        debug!("The Content-Type header was not appropriately set.");
                        Err(Error::UnsupportedMediaType)
                    }
                    StatusCode::TOO_MANY_REQUESTS => {
                        debug!("429 (Rate Limit Exceeded) This error indicates that the application has exhausted its maximum number of allotted API calls allowed for a given duration. If the client receives a Rate Limit Exceeded response the client should process this response and halt future API calls for the duration, in seconds, indicated by the Retry-After header. Applications that are in violation of this policy may have their access disabled to preserve the integrity of the API. Please refer to our Rate Limiting documentation below for more information on determining if you have been rate limited, and how to avoid it.");
                        debug!("Common Reasons:");
                        debug!("Unregulated API calls.");
                        Err(Error::TooManyRequests(rate_limit.timeout))
                    }
                    StatusCode::INTERNAL_SERVER_ERROR => {
                        debug!("500 (Internal Server Error) This error indicates an unexpected condition or exception which prevented the server from fulfilling an API request.");
                        Err(Error::InternalServerError)
                    }
                    StatusCode::SERVICE_UNAVAILABLE => {
                        debug!("503 (Service Unavailable) This error indicates the server is currently unavailable to handle requests because of an unknown reason. The Service Unavailable response implies a temporary condition which will be alleviated after some delay.");
                        Err(Error::ServiceUnavailable)
                    }
                    _ => Err(Error::Unknown),
                },
                None => {
                    debug!("An unexpected status code was returned.");
                    Err(Error::Unknown)
                }
            }
        }
    }
}

pub async fn request<T: DeserializeOwned>(url: &str, context: &ClientContext) -> Result<T> {
    let mut headers = HeaderMap::new();
    headers.append(
        "X-Riot-Token",
        context
            .options
            .api_key
            .parse()
            .expect("Unable to parse X-Riot-Token to header."),
    );

    let client = &context.http_client;

    let response =
        handle_response::<T>(client.get(url).headers(headers.clone()).send().await?).await;

    if context.options.retry {
        if let Err(Error::TooManyRequests(duration)) = response {
            tokio::time::sleep(duration).await;
            return handle_response::<T>(client.get(url).headers(headers.clone()).send().await?)
                .await;
        } else {
            return response;
        }
    } else {
        response
    }
}
