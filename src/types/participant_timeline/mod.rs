use super::{Lane, Role};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimeline {
    pub participant_id: i32,
    pub cs_diff_per_min_deltas: Option<HashMap<String, f32>>,
    pub damage_taken_per_min_deltas: Option<HashMap<String, f32>>,
    pub role: Role,
    pub damage_taken_diff_per_min_deltas: Option<HashMap<String, f32>>,
    pub xp_per_min_deltas: Option<HashMap<String, f32>>,
    pub xp_diff_per_min_deltas: Option<HashMap<String, f32>>,
    pub lane: Lane,
    pub creeps_per_min_deltas: Option<HashMap<String, f32>>,
    pub gold_per_min_deltas: Option<HashMap<String, f32>>,
}
