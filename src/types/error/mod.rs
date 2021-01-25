use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Status {
    pub message: String,
    pub status_code: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub status: Status,
}
