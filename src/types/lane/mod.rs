use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Lane {
    Mid,
    Middle,
    Top,
    Jungle,
    Bot,
    Bottom,
}
