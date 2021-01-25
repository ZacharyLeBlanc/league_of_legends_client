use strum_macros::{Display, EnumIter};

/// All regions available for the riot API.
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
