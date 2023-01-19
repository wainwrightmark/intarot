use itertools::Itertools;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::BTreeMap;
use strum::{EnumCount, IntoEnumIterator};
use yewdux::prelude::*;

use crate::data::prelude::{Guide, SpreadType};

use super::messages::ShufflePromptsMessage;

#[derive(PartialEq, Eq, Store)]
pub struct PromptsState {
    pub prompts: BTreeMap<(Guide, SpreadType), Vec<&'static str>>,
}

impl PromptsState {
    pub fn get_three_prompts(
        &self,
        guide: &Guide,
        spread: &SpreadType,
    ) -> (&'static str, &'static str, &'static str) {
        let vec = &self.prompts[&(*guide, *spread)];
        (vec[0], vec[1], vec[2])
    }
}

impl Default for PromptsState {
    fn default() -> Self {
        let mut guide_prompts: BTreeMap<Guide, Vec<&str>> = Default::default();
        let mut spread_prompts: BTreeMap<SpreadType, Vec<&str>> = Default::default();

        {
            let guide_prompts_data = include_str!("../tsv/guide_prompts.tsv");
            for l in guide_prompts_data.lines().skip(1) {
                if let Some((line, arr)) = deconstruct_line::<{ Guide::COUNT }>(l) {
                    for (guide, b) in Guide::iter().zip(arr) {
                        if b {
                            let entry = guide_prompts.entry(guide);
                            match entry {
                                std::collections::btree_map::Entry::Vacant(v) => {
                                    v.insert(vec![line]);
                                }
                                std::collections::btree_map::Entry::Occupied(mut o) => {
                                    o.get_mut().extend_one(line)
                                }
                            }
                        }
                    }
                }
            }
        }

        {
            let spread_prompts_data = include_str!("../tsv/spread_prompts.tsv");

            for l in spread_prompts_data.lines().skip(1) {
                if let Some((line, arr)) = deconstruct_line::<{ SpreadType::COUNT }>(l) {
                    for (st, b) in SpreadType::iter().zip(arr) {
                        if b {
                            let entry = spread_prompts.entry(st);
                            match entry {
                                std::collections::btree_map::Entry::Vacant(v) => {
                                    v.insert(vec![line]);
                                }
                                std::collections::btree_map::Entry::Occupied(mut o) => {
                                    o.get_mut().extend_one(line)
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut prompts: BTreeMap<(Guide, SpreadType), Vec<&'static str>> = Default::default();

        for (g, g_lines) in guide_prompts{
            for (st, st_lines) in spread_prompts.iter(){
                let lines = g_lines.iter().filter(|x| st_lines.contains(x)).cloned().collect_vec();
                prompts.insert((g, *st), lines);
            }
        }


         Self { prompts }
    }
}

fn deconstruct_line<const N: usize>(line: &'static str) -> Option<(&'static str, [bool; N])> {
    let mut split = line.splitn(N + 1, '\t');

    let Some(line) = split.next() else{
        return None;
    };

    let mut bools = split.map(|x| x.eq_ignore_ascii_case("true"));
    let arr = bools.next_chunk().unwrap();

    Some((line, arr))
}

impl Reducer<PromptsState> for ShufflePromptsMessage {
    fn apply(self, state: std::rc::Rc<PromptsState>) -> std::rc::Rc<PromptsState> {
        let mut rng = ThreadRng::default();

        let mut prompts = state.prompts.clone();

        for (_, v) in prompts.iter_mut() {
            v.shuffle(&mut rng);
        }
        PromptsState { prompts }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::PromptsState;

    #[test]
    pub fn test_guide_knowledge_spreads() {
        let state = PromptsState::default();

        for ((_guide, _spread), vec) in state.prompts.iter() {

            //println!("{guide} {spread}: {}", vec.len());
            assert!(vec.len() >= 1);
            for t in vec {
                assert!(!t.is_empty());
            }
        }
    }
}
