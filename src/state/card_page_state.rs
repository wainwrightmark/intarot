use std::collections::BTreeMap;
use std::rc::Rc;
use strum::EnumCount;
use yewdux::store::Reducer;
use yewdux::store::Store;

use crate::data::prelude::*;

use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug)]
#[store(storage = "local")]

pub struct CardPageState {
    pub user_data: QuestionData,
    pub cards: Rc<[Card; Card::COUNT]>,
    pub top_card_index: usize,

    pub last_hidden_card_index: usize,
    pub show_description: bool,
    pub has_shown_description: bool,
}

impl Default for CardPageState {
    fn default() -> Self {
        Self {
            top_card_index: 0,
            last_hidden_card_index: 1,
            cards: Card::get_random_ordering(),
            user_data: Default::default(),
            show_description: false,
            has_shown_description: false,
        }
    }
}

impl CardPageState {
    pub fn finish_card_index(&self) -> usize {
        if self.user_data.spread_type.is_ad_card_first() {
            0
        } else {
            self.user_data.spread_type.total_cards()
        }
    }
    pub fn draw_card(mut self) -> Self {
        if self.top_card_index < self.user_data.spread_type.total_cards() {
            self.top_card_index += 1;
            self.last_hidden_card_index = (self.top_card_index + 1)
                .min(self.user_data.spread_type.total_cards())
                .max(self.last_hidden_card_index);

            if self.top_card_index == self.finish_card_index() {
                self.show_description = true;
                self.has_shown_description = true;
            } else {
                self.show_description = false;
            }
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

        if index > self.finish_card_index(){
            index = index -1;
        }

        let card = self.cards[index];
        let key = MetaKey {
            star_sign: self.user_data.star_sign,
            guide: self.user_data.guide,
            card,
        };

        metas.get(&key).map(|x| *x)
    }

    pub fn is_top_card(&self, index: usize) -> bool {
        index == self.top_card_index
    }

    pub fn replace_card(mut self) -> Self {
        if self.top_card_index > 0 {
            self.top_card_index -= 1;
        }
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
        self.top_card_index > 0
    }

    pub fn can_draw(&self) -> bool {
        self.top_card_index < self.user_data.spread_type.total_cards()
    }

    pub fn reset(&mut self) {
        self.cards = Card::get_random_ordering();
        self.top_card_index = self.user_data.spread_type.initial_top_card_index();
        self.show_description = false;
        self.last_hidden_card_index = self.user_data.spread_type.initial_top_card_index() + 1;
        self.has_shown_description = false;
    }
}

impl Reducer<CardPageState> for DrawMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        (*state).clone().draw_card().into()
    }
}

impl Reducer<CardPageState> for ReplaceMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        (*state).clone().replace_card().into()
    }
}
impl Reducer<CardPageState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        (*state).clone().toggle_description().into()
    }
}

impl Reducer<CardPageState> for ResetMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        let mut state = (*state).clone();
        state.reset();
        state.into()
    }
}

impl Reducer<CardPageState> for MaybeChangeDataMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        if self.0 == state.user_data {
            state
        } else {
            let mut state = (*state).clone();
            state.user_data = self.0;
            state.reset();
            state.into()
        }
    }
}
