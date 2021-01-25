use super::{
    HighestAchievedSeasonTier, Mastery, ParticipantStats, ParticipantTimeline, Rune, Team,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub participant_id: i32,
    pub champion_id: i32,
    pub runes: Option<Vec<Rune>>,
    pub stats: ParticipantStats,
    pub team_id: Team,
    pub timeline: ParticipantTimeline,
    pub spell_1_id: i32,
    pub spell_2_id: i32,
    pub highest_achieved_season_tier: Option<HighestAchievedSeasonTier>,
    pub masteries: Option<Vec<Mastery>>,
}
