use super::LeagueItem;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueList {
    pub league_id: String,
    pub entries: Vec<LeagueItem>,
    pub tier: String,
    pub name: String,
    pub queue: String,
}
