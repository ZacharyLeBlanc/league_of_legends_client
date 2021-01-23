use super::PlayerDTO;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentityDTO {
    pub participant_id: i32,
    pub player: Option<PlayerDTO>,
}
