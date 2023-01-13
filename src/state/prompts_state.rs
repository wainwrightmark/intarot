use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::BTreeMap;
use yewdux::prelude::*;

use crate::data::prelude::Guide;

use super::messages::ShufflePromptsMessage;

#[derive(PartialEq, Eq, Store)]
pub struct PromptsState {
    pub prompts: BTreeMap<Guide, Vec<&'static str>>,
}

impl PromptsState {
    pub fn get_three_prompts(&self, guide: &Guide) -> (&'static str, &'static str, &'static str) {
        let vec = &self.prompts[guide];
        (vec[0], vec[1], vec[2])
    }
}

impl Default for PromptsState {
    fn default() -> Self {
        let data = include_str!("../tsv/prompts.tsv");

        let mut evelyn = vec![];
        let mut madame = vec![];
        let mut maledictus = vec![];

        for l in data.lines().skip(1) {
            if let Some((line, eve, mad, mal)) = deconstruct_line(l) {
                if eve {
                    evelyn.push(line)
                }
                if mad {
                    madame.push(line)
                }
                if mal {
                    maledictus.push(line)
                }
            }
        }

        let mut prompts: BTreeMap<Guide, Vec<&str>> = Default::default();
        prompts.insert(Guide::Evelyn, evelyn);
        prompts.insert(Guide::Madame, madame);
        prompts.insert(Guide::Maledictus, maledictus);

        Self { prompts }
    }
}

fn deconstruct_line(line: &'static str) -> Option<(&'static str, bool, bool, bool)> {
    let mut split = line.splitn(4, '\t');

    let Some(line) = split.next() else{
        return None;
    };

    let mut bools = split.map(|x| x.eq_ignore_ascii_case("true"));

    let a = bools.next().unwrap_or_default();
    let b = bools.next().unwrap_or_default();
    let c = bools.next().unwrap_or_default();

    Some((line, a, b, c))
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

        for (_, vec) in state.prompts.iter() {
            assert!(vec.len() >= 3);
            for t in vec{
                assert!(!t.is_empty());
            }
        }

    }

}