use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Display, EnumIter)]
pub enum Region {
    #[strum(serialize = "br1.api.riotgames.com")]
    Brazil,
    #[strum(serialize = "eun1.api.riotgames.com")]
    EuropeNordicAndEast,
    #[strum(serialize = "euw1.api.riotgames.com")]
    EuropeWest,
    #[strum(serialize = "jp1.api.riotgames.com")]
    Japan,
    #[strum(serialize = "kr.api.riotgames.com")]
    Korea,
    #[strum(serialize = "la1.api.riotgames.com")]
    LatinAmericaNorth,
    #[strum(serialize = "la2.api.riotgames.com")]
    LatinAmericaSouth,
    #[strum(serialize = "na1.api.riotgames.com")]
    NorthAmerica,
    #[strum(serialize = "oc1.api.riotgames.com")]
    Oceania,
    #[strum(serialize = "tr1.api.riotgames.com")]
    Turkey,
    #[strum(serialize = "ru.api.riotgames.com")]
    Russia,
}

#[derive(Clone, Display)]
pub enum Queue {
    #[strum(serialize = "RANKED_SOLO_5x5")]
    RankedSoloQueue,
    #[strum(serialize = "RANKED_FLEX_SR")]
    RankedFlexQueue,
    #[strum(serialize = "RANKED_FLEX_TT")]
    RankedFlexTwistedTreeline,
}

#[derive(Clone, Display)]
pub enum Division {
    #[strum(serialize = "I")]
    One,
    #[strum(serialize = "II")]
    Two,
    #[strum(serialize = "III")]
    Three,
    #[strum(serialize = "IV")]
    Four,
}

#[derive(Clone, Display)]
pub enum Tier {
    #[strum(serialize = "IRON")]
    Iron,
    #[strum(serialize = "BRONZE")]
    Bronze,
    #[strum(serialize = "SILVER")]
    Silver,
    #[strum(serialize = "GOLD")]
    Gold,
    #[strum(serialize = "PLATINUM")]
    Platinum,
    #[strum(serialize = "DIAMOND")]
    Diamond,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Team {
    Blue = 100,
    Red = 200,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameResult {
    #[serde(alias = "Win")]
    Won,
    #[serde(alias = "Fail")]
    Lost,
}

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
