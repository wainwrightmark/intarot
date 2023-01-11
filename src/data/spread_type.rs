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
    Browse,
}

impl SpreadType {
    /// total cards, excluding the finish card
    pub fn total_cards(&self) -> usize {
        match self {
            SpreadType::One => 1,
            SpreadType::Three => 3,
            SpreadType::Browse => 22,
        }
    }


    pub fn initial_top_card_index(&self) -> usize {
        match self {
            SpreadType::One => 1,
            SpreadType::Three => 3,
            SpreadType::Browse =>0,
        }
    }

    pub fn is_ad_card_first(&self) -> bool{
        match self {
            SpreadType::One => true,
            SpreadType::Three => true,
            SpreadType::Browse => false,
        }
    }
}

impl SpreadType {
    pub fn name(&self) -> &'static str {
        self.into()
    }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
