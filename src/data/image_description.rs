use std::str::FromStr;

use anyhow::bail;
use itertools::Itertools;
use yew::AttrValue;

use crate::data::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct ImageDescription {
    pub guide: Guide,
    pub card: Card,
    pub representation: AttrValue,
    pub guidance: AttrValue,
    pub specific_guidance: AttrValue,
}

impl Default for ImageDescription {
    fn default() -> Self {
        Self {
            guide: Guide::EvelynMusgrave, //whatever
            card: Card::Magician,         //whatever
            representation: Default::default(),
            guidance: Default::default(),
            specific_guidance: Default::default(),
        }
    }
}

impl ImageDescription {
    pub fn full_description(&self) -> AttrValue {
        format!(
            "{}\n{}\n{}",
            self.representation, self.guidance, self.specific_guidance
        )
        .into()
    }
}

impl FromStr for ImageDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ss_str, card_str, representation, guidance, specific_guidance) =
            s.split_terminator('\t').next_tuple().unwrap();

        let Some(guide) = Guide::from_str(ss_str).ok() else{
            bail!("Could not parse guide: {}", ss_str);
        };

        let Some(card) = Card::from_str(card_str).ok() else{
            bail!("Could not parse card: {}", card_str);
        };

        Ok(ImageDescription {
            guide,
            card,
            representation: representation.to_string().into(),
            guidance: guidance.clone().to_string().into(),
            specific_guidance: specific_guidance.clone().to_string().into(),
        })
    }
}
