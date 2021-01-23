use super::LeagueItemDTO;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueListDTO {
    pub league_id: String,
    pub entries: Vec<LeagueItemDTO>,
    pub tier: String,
    pub name: String,
    pub queue: String,
}
