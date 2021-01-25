use super::Player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
    pub participant_id: i32,
    pub player: Option<Player>,
}
