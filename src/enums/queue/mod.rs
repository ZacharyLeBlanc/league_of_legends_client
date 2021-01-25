use strum_macros::Display;

/// League of legends queues.
#[derive(Clone, Display)]
pub enum Queue {
    #[strum(serialize = "RANKED_SOLO_5x5")]
    RankedSoloQueue,
    #[strum(serialize = "RANKED_FLEX_SR")]
    RankedFlexQueue,
    #[strum(serialize = "RANKED_FLEX_TT")]
    RankedFlexTwistedTreeline,
}
