use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSeriesDTO {
    pub losses: i32,
    pub progress: String,
    pub target: i32,
    pub wins: i32,
}
