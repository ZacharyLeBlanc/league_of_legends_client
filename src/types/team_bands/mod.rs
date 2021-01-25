use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamBands {
    pub champion_id: i32,
    pub pick_turn: i32,
}
