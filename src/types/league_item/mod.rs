use super::MiniSeries;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueItem {
    pub fresh_blood: bool,
    pub wins: i32,
    pub summoner_name: String,
    pub mini_series: Option<MiniSeries>,
    pub inactive: bool,
    pub veteran: bool,
    pub hot_streak: bool,
    pub rank: String,
    pub league_points: i32,
    pub losses: i32,
    pub summoner_id: String,
}
