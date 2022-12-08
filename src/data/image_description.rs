use std::{borrow::Cow, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

use crate::data::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct ImageDescription {
    pub soothsayer: Soothsayer,
    pub card: Card,
    pub representation: Cow<'static, str>,
    pub guidance: Cow<'static, str>,
    pub specific_guidance: Cow<'static, str>,
}

impl Default for ImageDescription {
    fn default() -> Self {
        Self {
            soothsayer: Soothsayer::EvelynMusgrave, //whatever
            card: Card::Magician,                   //whatever
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
            representation: representation.to_string().into(),
            guidance: guidance.clone().to_string().into(),
            specific_guidance: specific_guidance.clone().to_string().into(),
        })
    }
}
