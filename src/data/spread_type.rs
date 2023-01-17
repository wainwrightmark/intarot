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
    One,
    Three,
    Love,
    Career,
    Seven,
    All,
}

impl SpreadType {
    /// total cards, excluding the finish card
    pub fn total_cards(&self) -> usize {
        match self {
            SpreadType::One => 1,
            SpreadType::Three => 3,
            SpreadType::Love => 3,
            SpreadType::Career => 3,
            SpreadType::Seven => 7,
            SpreadType::All => 22,
        }
    }

    pub fn initial_top_card_index(&self) -> usize {
        if self.is_ad_card_first(){
            self.total_cards()
        }
        else{
            0
        }
    }

    pub fn is_ad_card_first(&self) -> bool {
        match self {
            SpreadType::All => false,
            _=>true
        }
    }
}

impl SpreadType {
    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
