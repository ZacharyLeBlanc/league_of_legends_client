use strum_macros::Display;

/// Ranked divisions in league of legends.
#[derive(Clone, Display)]
pub enum Division {
    /// I
    #[strum(serialize = "I")]
    One,
    /// II
    #[strum(serialize = "II")]
    Two,
    /// III
    #[strum(serialize = "III")]
    Three,
    /// IV
    #[strum(serialize = "IV")]
    Four,
}
