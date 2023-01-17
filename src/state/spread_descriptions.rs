use std::{collections::BTreeMap, str::FromStr};

use crate::data::prelude::*;
use itertools::Itertools;
use yewdux::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpreadDescription {
    pub spread: SpreadType,
    pub dropdown_name: &'static str,
    pub evelyn: &'static str,
    pub madame: &'static str,
    pub maledictus: &'static str,
    pub slots: &'static str
}

impl SpreadDescription {

    pub fn guide_text(&self, guide: &Guide)-> &'static str{
        match guide {
            Guide::Evelyn => self.evelyn,
            Guide::Madame => self.madame,
            Guide::Maledictus => self.maledictus,
        }
    }

}

#[derive(PartialEq, Eq, Store)]
pub struct SpreadDescriptionState {
    pub descriptions: BTreeMap<SpreadType, SpreadDescription>,
}

impl SpreadDescriptionState {
    pub fn get_guide_spread_text(&self, data: &QuestionData) -> &'static str {
        self.descriptions
            .get(&(data.spread_type))
            .map(|x| x.guide_text(&data.guide))
            .unwrap_or_default()
    }
}

impl Default for SpreadDescriptionState {
    fn default() -> Self {
        let descriptions = {
            let data = include_str!("../tsv/spreads.tsv");
            let lines = data.lines();
            lines
                .skip(1) //skip headers
                .filter_map(|s| s.splitn(6, '\t').next_tuple())
                .filter_map(|(spread, dropdown_name, evelyn, madame, maledictus, slots)| {
                    let Ok(spread) = SpreadType::from_str(spread) else{
                        return None;
                    };
                    Some((spread, SpreadDescription { spread, dropdown_name, evelyn, madame, maledictus, slots }))
                })
                .collect()
        };

        Self {
            descriptions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpreadDescriptionState;
    use crate::data::prelude::*;
    use strum::EnumCount;

    #[test]
    pub fn test_spread_descriptions() {
        let state = SpreadDescriptionState::default();

        for (_, desc) in state.descriptions.iter() {
            assert!(!desc.dropdown_name.is_empty());
        }

        assert_eq!(SpreadType::COUNT, state.descriptions.len());
    }
}
