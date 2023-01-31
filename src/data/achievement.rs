use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};
#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    EnumCount,
    EnumIter,
    PartialOrd,
    Ord,
)]
pub enum Achievement {
    LandingClickBegin,
    LandingClickAdvanced,
    ChangeGuide,
    ChangeSpreadType,
    SwipeCard,
    ViewDescription,
    ClickSurvey,
    ClickDiscord,
    ClickAnotherReading,
    ClickShare,
}

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    EnumCount,
    EnumIter,
    PartialOrd,
    Ord,
)]
pub enum SocialPlatform {
    Facebook,
    Twitter,
    Discord,
    Instagram,
}
