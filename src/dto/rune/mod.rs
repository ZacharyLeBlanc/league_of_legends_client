use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneDTO {
    pub rune_id: i32,
    pub rank: i32,
}
