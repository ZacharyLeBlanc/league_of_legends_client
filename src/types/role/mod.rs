use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Role {
    Duo,
    None,
    Solo,
    #[serde(alias = "DUO_CARRY")]
    DuoCarry,
    #[serde(alias = "DUO_SUPPORT")]
    DuoSupport,
}
