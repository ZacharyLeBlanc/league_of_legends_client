use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameResult {
    #[serde(alias = "Win")]
    Won,
    #[serde(alias = "Fail")]
    Lost,
}
