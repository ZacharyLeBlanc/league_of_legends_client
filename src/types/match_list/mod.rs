use super::MatchReference;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchList {
    pub start_index: i32,
    pub total_games: i32,
    pub end_index: i32,
    pub matches: Vec<MatchReference>,
}
