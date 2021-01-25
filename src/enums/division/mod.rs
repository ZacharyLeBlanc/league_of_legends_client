use strum_macros::Display;

/// Ranked divisions in league of legends.
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
