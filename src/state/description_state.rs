use std::{collections::*, str::FromStr};
use crate::data::prelude::*;

use anyhow::anyhow;
use anyhow::bail;
use itertools::Itertools;
// use rand::Rng;
// use rand::seq::SliceRandom;
use yewdux::prelude::*;
#[derive(PartialEq, Eq, Store)]
pub struct ImageDescriptionState {
    pub descriptions: HashMap<(Card, DescriptionType), Vec<NewDescription>>,
}

impl ImageDescriptionState
 {
    //todo pass in rng
    pub fn get_layout_sections(&self, layout: &DescriptionLayout, card: &Card, guide: &Guide)-> Vec<&'static str>{

        let mut result : Vec<&'static str> = vec![];
        let types = layout.get_layout_types(*guide);

        for t in types{
            if let Some(v) = self.descriptions.get(&(*card, t)){
                if let Some(desc) = v.first(){
                    result.push(desc.text);
                }
            }
        }

        result
    }
}

impl Default for ImageDescriptionState {
    fn default() -> Self {
        let data = include_str!("../tsv/descriptions.tsv");
        let lines = data.lines();
        let descriptions: HashMap<_, _> = lines
            .skip(1) //skip headers
            .filter_map(|x| NewDescription::from_str(x).ok())
            //.map(|x| ImageDescription::from_str(x).unwrap())
            .map(|x| ((x.card, x.r#type), x))
            .into_group_map();

        Self { descriptions }
    }
}


#[derive(Debug, Clone, Copy,  PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NewDescription {
    pub card: Card,
    pub r#type: DescriptionType,
    pub text: &'static str,
}

impl FromStr for NewDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Box::leak(s.to_string().into_boxed_str());

        let (card_str, type_str, text) = s
            .split_terminator('\t')
            .next_tuple()
            .ok_or(anyhow!("Description did not have four sections"))?;

        let Some(r#type) = DescriptionType::from_str(type_str).ok() else{
            bail!("Could not parse description type: {}", type_str);
        };

        let Some(card) = Card::from_str(card_str).ok() else{
            bail!("Could not parse card: {}", card_str);
        };

        Ok(NewDescription { card, r#type, text })
    }
}


#[cfg(test)]
mod tests {
    use super::{DescriptionType, ImageDescriptionState};
    use crate::data::prelude::*;
    use strum::*;

    #[test]
    pub fn test_descriptions() {
        let state = ImageDescriptionState::default();

        for card in Card::iter() {
            for t in DescriptionType::iter() {
                if let Some(v) = state.descriptions.get(&(card, t)) {
                    if v.is_empty() {
                        panic!("Vec is empty for description for {} {t}", card.name());
                    }
                } else {
                    panic!("No description for {} {t}", card.name());
                }
            }
        }
    }
}
