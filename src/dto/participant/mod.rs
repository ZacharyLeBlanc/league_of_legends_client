use super::{MasteryDTO, ParticipantStatsDTO, ParticipantTimelineDTO, RuneDTO};
use crate::enums::{HighestAchievedSeasonTier, Team};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDTO {
    pub participant_id: i32,
    pub champion_id: i32,
    pub runes: Option<Vec<RuneDTO>>,
    pub stats: ParticipantStatsDTO,
    pub team_id: Team,
    pub timeline: ParticipantTimelineDTO,
    pub spell_1_id: i32,
    pub spell_2_id: i32,
    pub highest_achieved_season_tier: Option<HighestAchievedSeasonTier>,
    pub masteries: Option<Vec<MasteryDTO>>,
}
