use std::collections::BTreeMap;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use yewdux::prelude::*;

use crate::data::prelude::Guide;

use super::messages::ShufflePromptsMessage;

#[derive(PartialEq, Eq, Store)]
pub struct PromptsState {
    pub prompts: BTreeMap<Guide, Vec<&'static str>> ,
}

impl PromptsState{
    pub fn get_three_prompts(&self, guide: &Guide)-> (&'static str, &'static str, &'static str){
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

        for l in data.lines().skip(1){
            let (eve,mad,mal) = deconstruct_line(l);
            evelyn.extend(eve);
            madame.extend(mad);
            maledictus.extend(mal);
        }

        let mut prompts: BTreeMap<Guide, Vec<&str>> = Default::default();
        prompts.insert(Guide::Evelyn, evelyn);
        prompts.insert(Guide::Madame, madame);
        prompts.insert(Guide::Maledictus, maledictus);

        Self { prompts }
    }
}

fn deconstruct_line(line : &'static str)-> (Option<&'static str>, Option<&'static str>, Option<&'static str>){
    let (a,bc)= line.split_once('\t').unwrap_or_default();
    let (b,c) = bc.split_once('\t').unwrap_or_default();

    let a = if a.is_empty() {None} else {Some(a)};
    let b = if b.is_empty() {None} else {Some(b)};
    let c = if c.is_empty() {None} else {Some(c)};

    (a,b,c)
}



impl Reducer<PromptsState> for ShufflePromptsMessage{
    fn apply(self, state: std::rc::Rc<PromptsState>) -> std::rc::Rc<PromptsState> {
        let mut rng = ThreadRng::default();

        let mut prompts = state.prompts.clone();

        for (_, v) in prompts.iter_mut(){
            v.shuffle(&mut rng);
        }
        PromptsState{prompts}.into()
    }
}