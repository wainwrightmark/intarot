use itertools::Itertools;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use yewdux::prelude::*;

use super::messages::ShufflePromptsMessage;

#[derive(PartialEq, Eq, Store)]
pub struct PromptsState {
    pub prompts: Vec<&'static str>,
}

impl Default for PromptsState {
    fn default() -> Self {
        let data = include_str!("../text/prompts.txt");
        let prompts = data.lines().collect_vec();

        Self { prompts }
    }
}



impl Reducer<PromptsState> for ShufflePromptsMessage{
    fn apply(self, state: std::rc::Rc<PromptsState>) -> std::rc::Rc<PromptsState> {
        let mut rng = ThreadRng::default();

        let mut prompts = state.prompts.clone();
        prompts.shuffle(&mut rng);

        PromptsState{prompts}.into()
    }
}