use std::{collections::BTreeMap, str::FromStr};

use crate::data::prelude::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store)]
pub struct ImageDescriptionState {
    pub descriptions: BTreeMap<(Guide, Card), ImageDescription>,
}

impl Default for ImageDescriptionState {
    fn default() -> Self {
        let data = include_str!("../tsv/descriptions.tsv");
        let lines = data.lines();
        let descriptions: BTreeMap<_, _> = lines
            .skip(1) //skip headers
            .filter_map(|x| ImageDescription::from_str(x).ok())
            .map(|x| ((x.guide, x.card), x))
            .collect();

        Self { descriptions }
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumCount;
    use crate::data::prelude::*;
    use super::ImageDescriptionState;

    #[test]
    pub fn test_descriptions() {
        let state = ImageDescriptionState::default();
        assert_eq!(Guide::COUNT * Card::COUNT, state.descriptions.len());

        for (_, desc) in state.descriptions{
            assert!(!desc.guidance.is_empty());
            assert!(!desc.specific_guidance.is_empty());
            assert!(!desc.representation.is_empty());
        }
    }
}
