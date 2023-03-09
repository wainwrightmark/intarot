use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

#[derive(
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
    EnumProperty,
    EnumString,
    EnumIter,
    EnumCount,
    IntoStaticStr,
    Default,
    Display,
)]
#[strum(ascii_case_insensitive)]
pub enum SpreadType {
    #[default]
    #[strum(serialize = "One", serialize = "1")]
    One,
    #[strum(serialize = "Love", serialize = "l")]
    Love,
    #[strum(serialize = "Career", serialize = "c")]
    Career,
    #[strum(serialize = "DayAhead", serialize = "d")]
    DayAhead,
    #[strum(serialize = "Personal", serialize = "p")]
    Personal,
    #[strum(serialize = "Problem", serialize = "q")]
    Problem,
    #[strum(serialize = "Three", serialize = "3")]
    Three,
    #[strum(serialize = "Five", serialize = "5")]
    Five,
    #[strum(serialize = "All", serialize = "0")]
    All,
}

impl SpreadType {
    pub fn short_name(&self) -> &'static str {
        match self {
            SpreadType::One => "1",
            SpreadType::Love => "l",
            SpreadType::Career => "c",
            SpreadType::DayAhead => "d",
            SpreadType::Personal => "p",
            SpreadType::Problem => "q",
            SpreadType::Three => "3",
            SpreadType::Five => "5",
            SpreadType::All => "0",
        }
    }

    pub fn long_name(&self)-> &'static str{
        match self {
            SpreadType::One => "One Card",
            SpreadType::Love => "Love Reading",
            SpreadType::Career => "Career Reading",
            SpreadType::DayAhead => "Your Day Ahead",
            SpreadType::Personal => "Past Present Future",
            SpreadType::Problem => "A Problem",
            SpreadType::Three => "Three Cards",
            SpreadType::Five => "Five Cards",
            SpreadType::All => "Full Deck",
        }
    }

    /// total cards, excluding the finish card
    pub fn total_cards(&self) -> u8 {
        match self {
            SpreadType::One => 1,
            SpreadType::Three => 3,
            SpreadType::Love => 3,
            SpreadType::Career => 5,
            SpreadType::All => 22,
            SpreadType::DayAhead => 3,
            SpreadType::Personal => 3,
            SpreadType::Problem => 3,
            SpreadType::Five => 5,
        }
    }

    pub fn initial_top_card_index(&self) -> u8 {
        if self.is_ad_card_first() {
            self.total_cards()
        } else {
            0
        }
    }

    pub fn is_ad_card_first(&self) -> bool {
        !matches!(self, SpreadType::All)
    }
}

impl SpreadType {
    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
