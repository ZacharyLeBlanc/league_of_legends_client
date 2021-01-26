use strum_macros::Display;

/// League of legends queues.
#[derive(Clone, Display)]
pub enum Queue {
    /// RANKED_SOLO_5x5
    #[strum(serialize = "RANKED_SOLO_5x5")]
    RankedSoloQueue,
    /// RANKED_FLEX_SR
    #[strum(serialize = "RANKED_FLEX_SR")]
    RankedFlexQueue,
    /// RANKED_FLEX_TT
    #[strum(serialize = "RANKED_FLEX_TT")]
    RankedFlexTwistedTreeline,
}
