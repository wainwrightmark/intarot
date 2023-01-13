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
pub enum SpreadType {
    #[default]
    One,
    Three,
    All,
}

impl SpreadType {
    /// total cards, excluding the finish card
    pub fn total_cards(&self) -> usize {
        match self {
            SpreadType::One => 1,
            SpreadType::Three => 3,
            SpreadType::All => 22,
        }
    }

    pub fn initial_top_card_index(&self) -> usize {
        match self {
            SpreadType::One => 1,
            SpreadType::Three => 3,
            SpreadType::All => 0,
        }
    }

    pub fn is_ad_card_first(&self) -> bool {
        match self {
            SpreadType::One => true,
            SpreadType::Three => true,
            SpreadType::All => false,
        }
    }
}

impl SpreadType {
    pub fn name(&self) -> &'static str {
        match self {
            SpreadType::One => "One Card",
            SpreadType::Three => "Three Cards",
            SpreadType::All => "All Cards",
        }
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
