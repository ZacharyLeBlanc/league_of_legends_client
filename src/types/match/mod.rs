use super::{Participant, ParticipantIdentity, TeamStats};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub game_id: i64,
    pub participant_identities: Vec<ParticipantIdentity>,
    pub queue_id: i32,
    pub game_type: String,
    pub game_duration: i64,
    pub teams: Vec<TeamStats>,
    pub platform_id: String,
    pub game_creation: i64,
    pub season_id: i32,
    pub game_version: String,
    pub map_id: i32,
    pub game_mode: String,
    pub participants: Vec<Participant>,
}
