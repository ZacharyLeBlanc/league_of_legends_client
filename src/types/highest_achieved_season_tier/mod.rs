use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HighestAchievedSeasonTier {
    Challenger,
    Master,
    Diamond,
    Platinum,
    Gold,
    Silver,
    Bronze,
    Unranked,
}
