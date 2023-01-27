use std::{collections::BTreeMap, str::FromStr};

use crate::data::{description_layout::DescriptionLayout, prelude::*};
use itertools::Itertools;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpreadDescription {
    pub spread: SpreadType,
    pub dropdown_name: &'static str,
    pub evelyn: &'static str,
    pub madame: &'static str,
    pub maledictus: &'static str,
    pub slots: Vec<&'static str>,
    pub layout: Vec<DescriptionLayout>,
}

impl SpreadDescription {
    pub fn guide_text(&self, guide: &Guide) -> &'static str {
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

    pub fn try_get_slot(&self, data: &QuestionData, index: usize) -> Option<&'static str> {
        let index = if data.spread_type.is_ad_card_first() {
            index.checked_sub(1)?
        } else {
            index
        };
        self.descriptions
            .get(&(data.spread_type))
            .and_then(|x| x.slots.get(index).cloned())
            .and_then(|x| if x.is_empty() { None } else { Some(x) })
    }

    pub fn try_get_layout(&self, data: &QuestionData, index: usize) -> Option<DescriptionLayout> {
        let index = if data.spread_type.is_ad_card_first() {
            index.checked_sub(1)?
        } else {
            index
        };
        self.descriptions
            .get(&(data.spread_type))
            .and_then(|x| x.layout.get(index).cloned())
    }
}

impl Default for SpreadDescriptionState {
    fn default() -> Self {
        let descriptions = {
            let data = include_str!("../tsv/spreads.tsv");
            let lines = data.lines();
            lines
                .skip(1) //skip headers
                .filter_map(|s| s.splitn(7, '\t').next_tuple())
                .filter_map(
                    |(spread, dropdown_name, evelyn, madame, maledictus, slots, layout)| {
                        let Ok(spread) = SpreadType::from_str(spread) else{
                        return None;
                    };
                        let slots = slots
                            .split_terminator(';')
                            .map(|x| x.trim())
                            .rev()
                            .collect_vec();

                        let layout = layout
                            .split_terminator(';')
                            .map(|s| s.trim())
                            .filter(|x| !x.is_empty())
                            .map(|s| DescriptionLayout::from_str(s).unwrap())
                            .rev()
                            .collect_vec();

                        Some((
                            spread,
                            SpreadDescription {
                                spread,
                                dropdown_name,
                                evelyn,
                                madame,
                                maledictus,
                                slots,
                                layout,
                            },
                        ))
                    },
                )
                .collect()
        };

        Self { descriptions }
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
