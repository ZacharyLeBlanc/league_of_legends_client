use reqwest::header::HeaderMap;
use std::{
    fmt::{Display, Formatter, Result},
    time::Duration,
};

#[derive(Debug, Default)]
pub struct Rate {
    pub requests: Option<u16>,
    pub seconds: Option<u16>,
}

#[derive(Debug, Default)]
pub struct RateLimit {
    pub app_rate_limit: Option<Vec<Rate>>,
    pub app_rate_limit_count: Option<Vec<Rate>>,
    pub method_rate_limit: Option<Vec<Rate>>,
    pub method_rate_limit_count: Option<Vec<Rate>>,
    pub timeout: Duration,
}

fn parse_rate_limit(header_string: &str) -> Vec<Rate> {
    header_string
        .split(',')
        .map(|rate_string| {
            let mut rate = Rate::default();
            rate_string
                .split(':')
                .enumerate()
                .for_each(|(index, part)| match index {
                    0 => {
                        if let Ok(number_of_requests) = part.parse::<u16>() {
                            rate.requests = Some(number_of_requests);
                        }
                    }
                    1 => {
                        if let Ok(number_of_seconds) = part.parse::<u16>() {
                            rate.seconds = Some(number_of_seconds);
                        }
                    }
                    _ => {}
                });
            rate
        })
        .collect()
}

impl From<&HeaderMap> for RateLimit {
    fn from(response_headers: &HeaderMap) -> RateLimit {
        let mut rate_limit = RateLimit::default();
        if let Some(app_rate_limit_header) = response_headers.get("X-App-Rate-Limit") {
            if let Ok(app_rate_limit_header_string) = app_rate_limit_header.to_str() {
                rate_limit.app_rate_limit = Some(parse_rate_limit(app_rate_limit_header_string));
            }
        }

        if let Some(app_rate_limit_count_header) = response_headers.get("X-App-Rate-Limit-Count") {
            if let Ok(app_rate_limit_count_header_string) = app_rate_limit_count_header.to_str() {
                rate_limit.app_rate_limit_count =
                    Some(parse_rate_limit(app_rate_limit_count_header_string));
            }
        }

        if let Some(method_rate_limit_header) = response_headers.get("X-Method-Rate-Limit") {
            if let Ok(method_rate_limit_header_string) = method_rate_limit_header.to_str() {
                rate_limit.method_rate_limit =
                    Some(parse_rate_limit(method_rate_limit_header_string));
            }
        }

        if let Some(method_rate_limit_count_header) =
            response_headers.get("X-Method-Rate-Limit-Count")
        {
            if let Ok(method_rate_limit_count_header_string) =
                method_rate_limit_count_header.to_str()
            {
                rate_limit.method_rate_limit_count =
                    Some(parse_rate_limit(method_rate_limit_count_header_string));
            }
        }

        if let Some(retry_after_header) = response_headers.get("Retry-After") {
            if let Ok(retry_after_header_string) = retry_after_header.to_str() {
                if let Ok(seconds) = retry_after_header_string.parse::<u64>() {
                    rate_limit.timeout = Duration::from_secs(seconds);
                }
            }
        }

        rate_limit
    }
}

impl Display for RateLimit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut string = String::new();
        if let (Some(app_rate_limit), Some(app_rate_limit_count)) =
            (&self.app_rate_limit, &self.app_rate_limit_count)
        {
            string.push_str("(App rate limit):");
            for (rate_limit, rate_limit_count) in
                app_rate_limit.iter().zip(app_rate_limit_count.iter())
            {
                if let (Some(limit_requests), Some(limit_count_requests), Some(limit_seconds)) = (
                    rate_limit.requests,
                    rate_limit_count.requests,
                    rate_limit.seconds,
                ) {
                    string.push_str(&format!(
                        " You've used {}/{} requests per {} second(s). |",
                        limit_count_requests, limit_requests, limit_seconds
                    ));
                }
            }
        }

        if let (Some(method_rate_limit), Some(method_rate_limit_count)) =
            (&self.method_rate_limit, &self.method_rate_limit_count)
        {
            string.push_str("(Method rate limit):");
            for (rate_limit, rate_limit_count) in
                method_rate_limit.iter().zip(method_rate_limit_count.iter())
            {
                if let (Some(limit_requests), Some(limit_count_requests), Some(limit_seconds)) = (
                    rate_limit.requests,
                    rate_limit_count.requests,
                    rate_limit.seconds,
                ) {
                    string.push_str(&format!(
                        " You've used {}/{} requests per {} second(s). |",
                        limit_count_requests, limit_requests, limit_seconds
                    ));
                }
            }
        }
        write!(f, "{}", string)
    }
}
