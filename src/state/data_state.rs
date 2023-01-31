use num_traits::cast::FromPrimitive;
use std::collections::HashMap;
use std::rc::Rc;
use yewdux::prelude::Dispatch;
use yewdux::store::Reducer;
use yewdux::store::Store;

use crate::data::prelude::*;

use super::achievements_state::AchievementsState;
use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug)]
#[store(storage = "local")]
pub struct DataState {
    pub question_data: QuestionData,
    pub cards_permutation: Perm,
    pub top_card_index: usize,

    pub last_hidden_card_index: usize,
    pub show_description: bool,
    pub has_shown_description: bool,
}

impl Default for DataState {
    fn default() -> Self {
        Self {
            top_card_index: 0,
            last_hidden_card_index: 1,
            cards_permutation: Card::get_random_ordering(),
            question_data: Default::default(),
            show_description: false,
            has_shown_description: false,
        }
    }
}

impl DataState {
    pub fn finish_card_index(&self) -> usize {
        if self.question_data.spread_type.is_ad_card_first() {
            0
        } else {
            self.question_data.spread_type.total_cards()
        }
    }
    pub fn next_card(mut self) -> Self {
        self.top_card_index =
            (self.top_card_index + 1) % (self.question_data.spread_type.total_cards() + 1);
        self.last_hidden_card_index = (self.top_card_index + 1)
            .max(self.last_hidden_card_index)
            .min(self.question_data.spread_type.total_cards()); //DO NOT USE CLAMP

        if self.top_card_index == self.finish_card_index() {
            self.show_description = true;
            self.has_shown_description = true;
        } else {
            self.show_description = false;
        }
        self
    }

    pub fn get_image_meta<'a>(
        &self,
        mut index: usize,
        metas: &'a HashMap<MetaKey, Vec<ImageMeta>>,
    ) -> Option<&'a ImageMeta> {
        if index == self.finish_card_index() {
            return None;
        }

        if index > self.finish_card_index() {
            index -= 1;
        }

        let card = self.cards_permutation.element_at_index(index, |x| {
            Card::from_u8(x as u8).expect("Could not make card from u8")
        });
        let key = MetaKey {
            guide: self.question_data.guide,
            card,
        };
        let vec = metas.get(&key)?;

        if vec.is_empty() {
            None
        } else if vec.len() == 1 {
            vec.get(0)
        } else {
            let variant_index = self.variant_index();
            let variant_index = variant_index % (vec.len() as u64);
            vec.get(variant_index as usize)
        }
    }

    fn variant_index(&self) -> u64 {
        let be_bytes = self.cards_permutation.0.to_be_bytes();
        let mut arr = [0u8; 8];
        arr.clone_from_slice(&be_bytes[8..]);
        //log::info!("{arr:?}");

        u64::from_be_bytes(arr)
    }

    pub fn is_top_card(&self, index: usize) -> bool {
        index == self.top_card_index
    }

    pub fn previous_card(mut self) -> Self {
        self.top_card_index = (self.top_card_index + self.question_data.spread_type.total_cards())
            % (self.question_data.spread_type.total_cards() + 1);
        if self.top_card_index == self.finish_card_index() {
            self.show_description = true;
            self.has_shown_description = true;
        } else {
            self.show_description = false;
        }

        self.last_hidden_card_index = (self.top_card_index + 1)
            .max(self.last_hidden_card_index)
            .min(self.question_data.spread_type.total_cards()); //DO NOT USE CLAMP
        self
    }

    pub fn toggle_description(mut self) -> Self {
        self.show_description = !self.show_description;
        self.has_shown_description = true;
        self
    }

    pub fn can_previous(&self) -> bool {
        self.top_card_index > 0
    }

    pub fn can_draw(&self) -> bool {
        self.top_card_index < self.question_data.spread_type.total_cards()
    }

    pub fn spread_src(&self, metas: &HashMap<MetaKey, Vec<ImageMeta>>) -> SrcData {
        let card_name = self
            .get_image_meta(0, metas)
            .map(|x| x.file_name)
            .unwrap_or_default();
        SrcData::Spread {
            card_name,
            question_data: self.question_data,
            perm: self.cards_permutation,
        }
    }

    pub fn reset(&mut self) {
        self.cards_permutation = Card::get_random_ordering();
        self.back_to_top();
    }

    pub fn back_to_top(&mut self) {
        self.top_card_index = self.question_data.spread_type.initial_top_card_index();
        self.show_description = false;
        self.last_hidden_card_index = self.question_data.spread_type.initial_top_card_index() + 1;
        self.has_shown_description = false;
    }
}

impl Reducer<DataState> for DrawMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        Dispatch::<AchievementsState>::new()
            .apply(AchievementEarnedMessage(Achievement::SwipeCard));
        (*state).clone().next_card().into()
    }
}

impl Reducer<DataState> for ReplaceMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        Dispatch::<AchievementsState>::new()
            .apply(AchievementEarnedMessage(Achievement::SwipeCard));
        (*state).clone().previous_card().into()
    }
}
impl Reducer<DataState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        Dispatch::<AchievementsState>::new()
            .apply(AchievementEarnedMessage(Achievement::ViewDescription));
        (*state).clone().toggle_description().into()
    }
}

impl Reducer<DataState> for ResetMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();
        state.reset();
        state.into()
    }
}

impl Reducer<DataState> for LoadSpreadMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();
        state.question_data = self.0;
        state.cards_permutation = self.1;

        state.back_to_top();

        state.into()
    }
}

impl Reducer<DataState> for ChangeSpreadTypeMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();

        if state.question_data.spread_type != self.0 {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ChangeSpreadType));
        }
        state.question_data.spread_type = self.0;
        state.reset();
        state.into()
    }
}

impl Reducer<DataState> for ChangeGuideMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();
        if state.question_data.guide != self.0 {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ChangeGuide));
        }
        state.question_data.guide = self.0;
        state.reset();
        state.into()
    }
}
