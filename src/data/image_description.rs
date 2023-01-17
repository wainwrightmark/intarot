use std::str::FromStr;

use anyhow::{anyhow, bail};
use itertools::Itertools;
use yew::AttrValue;

use crate::data::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ImageDescription {
    pub guide: Guide,
    pub card: Card,
    pub representation: &'static str,
    pub guidance: &'static str,
    pub user_representation: &'static str,
    pub agent_representation: &'static str,
    pub guide_interpretation: &'static str,
}

impl ImageDescription {
    pub fn full_description(&self) -> AttrValue {
        format!(
            "{}\n{}\n{}",
            self.representation, self.guidance, self.guide_interpretation
        )
        .into()
    }
}

impl FromStr for ImageDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Box::leak(s.to_string().into_boxed_str());

        let (ss_str, card_str, representation, guidance, user_representation, agent_representation, guide_interpretation) = s
            .split_terminator('\t')
            .next_tuple()
            .ok_or(anyhow!("Description did not have four sections"))?;

        let Some(guide) = Guide::from_str(ss_str).ok() else{
            bail!("Could not parse guide: {}", ss_str);
        };

        let Some(card) = Card::from_str(card_str).ok() else{
            bail!("Could not parse card: {}", card_str);
        };

        Ok(ImageDescription {
            guide,
            card,
            representation,
            guidance,
            user_representation,
            agent_representation,
            guide_interpretation
        })
    }
}
