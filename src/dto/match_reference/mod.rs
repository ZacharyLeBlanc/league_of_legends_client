use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchReferenceDTO {
    pub game_id: i64,
    pub role: String,
    pub season: i32,
    pub platform_id: String,
    pub champion: i32,
    pub queue: i32,
    pub lane: String,
    pub timestamp: i64,
}
