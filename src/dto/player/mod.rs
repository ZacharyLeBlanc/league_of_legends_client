use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDTO {
    pub profile_icon: i32,
    pub account_id: String,
    pub match_history_uri: String,
    pub current_account_id: String,
    pub current_platform_id: String,
    pub summoner_name: String,
    pub summoner_id: String,
    pub platform_id: String,
}
