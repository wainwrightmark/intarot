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
            SpreadType::All =>0,
        }
    }

    pub fn is_ad_card_first(&self) -> bool{
        match self {
            SpreadType::One => true,
            SpreadType::Three => true,
            SpreadType::All => false,
        }
    }
}

impl SpreadType {
    pub fn name(&self) -> &'static str {
        match self{
            SpreadType::One => "One Card",
            SpreadType::Three => "Three Cards",
            SpreadType::All => "All Cards",
        }
    }

    // pub fn description(&self)-> &'static str{
    //     match self {
    //         SpreadType::One => "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed faucibus ornare orci, eu molestie dolor imperdiet id. Nullam suscipit fringilla posuere. Integer pharetra condimentum massa eu aliquam.",
    //         SpreadType::Three => "Decide in advance what these will represent. Some choose the past, present and future. Others choose problem, cause and solution. The three card spread can tell you almost anything. A spread for love might represent you, the other person and the nature of your relationship.",
    //         SpreadType::Browse => "Vestibulum efficitur lorem eget leo aliquet eleifend. Sed ac fringilla libero, a rutrum massa. Nulla pellentesque sit amet nulla eu condimentum. Duis orci odio, eleifend id sapien luctus, iaculis malesuada ante. Aliquam aliquam dignissim scelerisque. Morbi diam nibh, fermentum sed aliquam vitae, molestie sit amet diam.",
    //     }
    // }

    pub fn repr(&self) -> &'static str {
        self.into()
    }
}
