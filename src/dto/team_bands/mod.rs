use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamBandsDTO {
    pub champion_id: i32,
    pub pick_turn: i32,
}
