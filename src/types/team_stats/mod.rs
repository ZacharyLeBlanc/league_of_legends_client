use super::{GameResult, Team, TeamBands};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
    pub tower_kills: i32,
    pub rift_herald_kills: i32,
    pub first_blood: bool,
    pub inhibitor_kills: i32,
    pub bans: Option<Vec<TeamBands>>,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub dominion_victory_score: i32,
    pub dragon_kills: i32,
    pub baron_kills: i32,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub vilemaw_kills: i32,
    pub first_rift_herald: bool,
    pub team_id: Team,
    pub win: GameResult,
}
