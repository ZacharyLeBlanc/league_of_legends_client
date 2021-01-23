use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MasteryDTO {
    pub rank: i32,
    pub mastery_id: i32,
}
