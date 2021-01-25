use strum_macros::Display;

/// Ranked tiers in league of legends.
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
