use std::collections::BTreeMap;
use std::rc::Rc;
use strum::EnumCount;
use yewdux::store::Reducer;
use yewdux::store::Store;

use crate::data::prelude::*;

use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug)]
#[store(storage = "local")]

pub struct DataState {
    pub question_data: QuestionData,
    pub cards: Rc<[Card; Card::COUNT]>,
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
            cards: Card::get_random_ordering(),
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
        self.top_card_index = (self.top_card_index  + 1) % (self.question_data.spread_type.total_cards() + 1);
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

    pub fn get_image_meta(
        &self,
        mut index: usize,
        metas: &BTreeMap<MetaKey, ImageMeta>,
    ) -> Option<ImageMeta> {
        if index == self.finish_card_index() {
            return None;
        }

        if index > self.finish_card_index() {
            index -= 1;
        }

        let card = self.cards[index];
        let key = MetaKey {
            guide: self.question_data.guide,
            card,
        };

        metas.get(&key).copied()
    }

    pub fn is_top_card(&self, index: usize) -> bool {
        index == self.top_card_index
    }

    pub fn previous_card(mut self) -> Self {
        self.top_card_index = (self.top_card_index  + self.question_data.spread_type.total_cards()) % (self.question_data.spread_type.total_cards() + 1);
        if self.top_card_index == self.finish_card_index() {
            self.show_description = true;
            self.has_shown_description = true;
        } else {
            self.show_description = false;
        }
        self
    }

    pub fn toggle_description(mut self) -> Self {
        self.show_description = !self.show_description;
        self.has_shown_description = true;
        self
    }

    pub fn can_previous(&self) -> bool {
        // self.top_card_index > 0
        true
    }

    pub fn can_draw(&self) -> bool {
        // self.top_card_index < self.question_data.spread_type.total_cards()
        true
    }

    pub fn reset(&mut self) {
        self.cards = Card::get_random_ordering();
        self.top_card_index = self.question_data.spread_type.initial_top_card_index();
        self.show_description = false;
        self.last_hidden_card_index = self.question_data.spread_type.initial_top_card_index() + 1;
        self.has_shown_description = false;
    }
}

impl Reducer<DataState> for DrawMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        (*state).clone().next_card().into()
    }
}

impl Reducer<DataState> for ReplaceMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        (*state).clone().previous_card().into()
    }
}
impl Reducer<DataState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
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

impl Reducer<DataState> for MaybeChangeDataMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        if self.0 == state.question_data {
            state
        } else {
            let mut state = (*state).clone();
            state.question_data = self.0;
            state.reset();
            state.into()
        }
    }
}
