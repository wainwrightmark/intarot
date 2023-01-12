use std::{collections::BTreeMap, str::FromStr};

use crate::data::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KnowledgeDescription {
    pub text: &'static str,
}

#[derive(PartialEq, Eq, Store)]
pub struct GuideKnowledgeState {
    pub guide_spreads: BTreeMap<(Guide, SpreadType), KnowledgeDescription>,
    pub guide_star_signs: BTreeMap<(Guide, Option<StarSign>), KnowledgeDescription>,
}

impl GuideKnowledgeState{
    pub fn get_guide_spread_text(&self, data: &QuestionData)-> &'static str{
        self.guide_spreads.get(&(data.guide, data.spread_type)).map(|x|x.text).unwrap_or_default()
    }

    pub fn get_guide_star_sign_text(&self, data: &QuestionData)-> &'static str{
        self.guide_star_signs.get(&(data.guide, data.star_sign)).map(|x|x.text).unwrap_or_default()
    }
}

impl Default for GuideKnowledgeState {
    fn default() -> Self {
        let guide_spreads = {
            let data = include_str!("../tsv/guide_spreads.tsv");
            let lines = data.lines();
            lines
                .skip(1) //skip headers
                //.filter_map(|x| x.ok())
                .map(|x| split_into_three(x))
                .filter_map(|(guide, spread, text)| {
                    let Ok(guide) = Guide::from_str(guide) else{
                        return None;
                    };
                    let Ok(spread) = SpreadType::from_str(spread) else{
                        return None;
                    };
                    Some(((guide, spread), KnowledgeDescription { text }))
                })
                .collect()
        };

        let guide_star_signs = {
            let data = include_str!("../tsv/guide_star_signs.tsv");
            let lines = data.lines();
            lines
                .skip(1) //skip headers
                //.filter_map(|x| x.ok())
                .map(|x| split_into_three(x))
                .filter_map(|(guide, sign, text)| {
                    let Ok(guide) = Guide::from_str(guide) else{
                        return None;
                    };
                    let sign =  StarSign::from_str(sign).ok();
                    Some(((guide, sign), KnowledgeDescription { text }))
                })
                .collect()
        };

        Self {
            guide_spreads,
            guide_star_signs,
        }
    }
}

fn split_into_three(s: &'static str) -> (&'static str, &'static str, &'static str) {
    let (one, rest) = s.split_once('\t').unwrap();
    let (two, three) = rest.split_once('\t').unwrap();

    (one, two, three)
}
