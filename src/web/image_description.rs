use std::str::FromStr;

use anyhow::bail;
use itertools::Itertools;

use crate::web::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct ImageDescription {
    pub soothsayer: Soothsayer,
    pub card: Card,
    pub representation: String,
    pub guidance: String,
    pub specific_guidance: String,
}

impl Default for ImageDescription {
    fn default() -> Self {
        Self {
            soothsayer: Soothsayer::EvelynMusgrave, //whatever
            card: Card::Magician, //whatever
            representation: Default::default(),
            guidance: Default::default(),
            specific_guidance: Default::default(),
        }
    }
}

impl FromStr for ImageDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ss_str, card_str, representation, guidance, specific_guidance) =
            s.split_terminator('\t').next_tuple().unwrap();
        
        let Some(soothsayer) = Soothsayer::from_str(ss_str).ok() else{
            bail!("Could not parse soothsayer: {}", ss_str);
        };
        
        let Some(card) = Card::from_str(card_str).ok() else{
            bail!("Could not parse card: {}", card_str);
        };

        Ok(ImageDescription {
            soothsayer,
            card,
            representation: representation.to_string(),
            guidance: guidance.to_string(),
            specific_guidance: specific_guidance.to_string(),
        })
    }
}
