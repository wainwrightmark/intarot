use num_traits::cast::FromPrimitive;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use yewdux::prelude::Dispatch;
use yewdux::store::Reducer;
use yewdux::store::Store;

use crate::data::image_data::ImageData;
use crate::data::image_data::ImageType;
use crate::data::prelude::*;

use super::achievements_state::AchievementsState;
use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Debug)]
pub struct CustomSpread {
    pub cards: Vec<ImageData>,
}

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug)]
#[store(storage = "local")]
pub struct DataState {
    pub question_data: QuestionData,
    pub perm: Perm,
    pub top_card_index: u8,
    pub cards_facing_up: u8,

    pub last_hidden_card_index: u8,
    pub show_description: bool,
    pub has_shown_description: bool,

    pub custom_spread: Option<Arc<CustomSpread>>,
}

impl Default for DataState {
    fn default() -> Self {
        Self {
            top_card_index: 0,
            last_hidden_card_index: 1,
            perm: Card::get_random_ordering(),
            question_data: Default::default(),
            show_description: false,
            has_shown_description: false,
            custom_spread: None,
            cards_facing_up: 0,
        }
    }
}

impl DataState {
    pub fn ad_card_index(&self) -> u8 {
        if self.question_data.spread_type.is_ad_card_first() {
            0
        } else {
            self.total_cards()
        }
    }

    pub fn total_cards(&self) -> u8 {
        match &self.custom_spread {
            Some(custom) => custom.cards.len() as u8,
            None => self.question_data.spread_type.total_cards(),
        }
    }

    pub fn get_image_meta(
        &self,
        mut index: u8,
        metas: &HashMap<MetaKey, Vec<ImageMeta>>,
    ) -> Option<ImageMeta> {
        if index == self.ad_card_index() {
            return None;
        }

        if index > self.ad_card_index() {
            index -= 1;
        }

        let card = self.perm.element_at_index(index, |x| {
            Card::from_u8(x).expect("Could not make card from u8")
        });

        let guide = self.question_data.guide;

        if let Some(custom) = &self.custom_spread {
            if index >= custom.cards.len() as u8 {
                return None;
            } else {
                let image_data = custom.cards[index as usize].clone();
                return Some(ImageMeta {
                    guide,
                    card,
                    image_data,
                });
            }
        }

        let key = MetaKey { guide, card };
        let result: Option<ImageMeta>;
        if let Some(vec) = metas.get(&key) {
            result = if vec.is_empty() {
                None
            } else if vec.len() == 1 {
                vec.get(0).cloned()
            } else {
                let variant_index = Self::variant_index(self.perm);
                let variant_index = variant_index % (vec.len() as u64);
                vec.get(variant_index as usize).cloned()
            };
        } else {
            result = None;
        };

        if result.is_none() {
            return Some(ImageMeta {
                image_data: ImageData::placeholder(guide, card),
                guide,
                card,
            });
        }
        result
    }

    fn variant_index(perm: Perm) -> u64 {
        let be_bytes = perm.inner().to_be_bytes();
        let mut arr = [0u8; 8];
        arr.clone_from_slice(&be_bytes[8..]);
        //log::info!("{arr:?}");

        u64::from_be_bytes(arr)
    }

    pub fn is_top_card(&self, index: u8) -> bool {
        index == self.top_card_index
    }

    /// If the card front face is up
    pub fn is_card_facing_up(&self, index: u8) -> bool {
        if index == self.ad_card_index() {
            //log::info!("Index {index} is flipped (ad card)");
            return true;
        }
        if self.question_data.spread_type.is_ad_card_first() {
            self.total_cards().saturating_sub(index) < self.cards_facing_up
        } else {
            index < self.cards_facing_up
        }
    }

    pub fn next_card(mut self) -> Self {
        if self.is_card_facing_up(self.top_card_index)
            || self.question_data.spread_type.is_ad_card_first()
        {
            self.top_card_index = (self.top_card_index + 1) % (self.total_cards() + 1);
            self.last_hidden_card_index = (self.top_card_index + 1)
                .max(self.last_hidden_card_index)
                .min(self.total_cards()); //DO NOT USE CLAMP

            if self.top_card_index == self.ad_card_index() {
                self.show_description = true;
                self.has_shown_description = true;
            } else {
                self.show_description = false;
            }
            self
        } else {
            self.cards_facing_up += 1;
            self
        }
    }

    pub fn previous_card(mut self) -> Self {
        if self.is_card_facing_up(self.top_card_index)
            || !self.question_data.spread_type.is_ad_card_first()
        {
            self.top_card_index =
                (self.top_card_index + self.total_cards()) % (self.total_cards() + 1);
            if self.top_card_index == self.ad_card_index() {
                self.show_description = true;
                self.has_shown_description = true;
            } else {
                self.show_description = false;
            }

            self.last_hidden_card_index = (self.top_card_index + 1)
                .max(self.last_hidden_card_index)
                .min(self.total_cards()); //DO NOT USE CLAMP
            self
        } else {
            self.cards_facing_up += 1;
            self
        }
    }

    pub fn toggle_description(mut self) -> Self {
        //log::info!("Toggle Desc");
        if self.is_card_facing_up(self.top_card_index) {
            self.show_description = !self.show_description;
            self.has_shown_description = true;
        } else {
            self.cards_facing_up += 1;
        }
        self
    }

    pub fn can_replace(&self) -> bool {
        self.top_card_index > 0
    }

    pub fn can_draw(&self) -> bool {
        self.top_card_index < self.total_cards()
    }

    pub fn spread_src(&self, metas: &HashMap<MetaKey, Vec<ImageMeta>>) -> SrcData {
        let spread_option = self
            .get_image_meta(self.initial_top_card_index(), metas)
            .map(|image_meta| image_meta.image_data)
            .map(|share_img| SpreadShare {
                question_data: self.question_data,
                perm: self.perm,
                share_img,
            });

        SrcData {
            spread_option,
            image: ImageData {
                id: self.question_data.guide.ad_image_data().to_string().into(),
                image_type: ImageType::Final,
                alt: "Plain Card".to_string(),
            },
        }
    }

    pub fn reset(&mut self) {
        self.perm = Card::get_random_ordering();
        self.custom_spread = None;
        self.back_to_top();
    }

    fn initial_top_card_index(&self) -> u8 {
        match &self.custom_spread {
            Some(custom_spread) => custom_spread.cards.len() as u8,
            None => self.question_data.spread_type.initial_top_card_index(),
        }
    }

    pub fn back_to_top(&mut self) {
        self.cards_facing_up = 0;
        self.top_card_index = self.initial_top_card_index();
        self.show_description = false;
        self.last_hidden_card_index = self.initial_top_card_index() + 1;
        self.has_shown_description = false;
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct DrawMessage {}

impl Reducer<DataState> for DrawMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        Dispatch::<AchievementsState>::new()
            .apply(AchievementEarnedMessage(Achievement::SwipeCard));
        (*state).clone().next_card().into()
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ReplaceMessage {}

impl Reducer<DataState> for ReplaceMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        Dispatch::<AchievementsState>::new()
            .apply(AchievementEarnedMessage(Achievement::SwipeCard));
        (*state).clone().previous_card().into()
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleDescriptionMessage {}

impl Reducer<DataState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        Dispatch::<AchievementsState>::new()
            .apply(AchievementEarnedMessage(Achievement::ViewDescription));
        (*state).clone().toggle_description().into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ResetMessage;

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
        state.perm = self.1;
        state.custom_spread = None;

        state.back_to_top();

        state.into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ChangeSpreadTypeMessage(pub SpreadType);

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

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ChangeGuideMessage(pub Guide);

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

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct SetPermutationMessage {
    pub permutation: Perm,
}

impl Reducer<DataState> for SetPermutationMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();
        state.perm = self.permutation;
        state.back_to_top();
        state.into()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SetCustomSpreadMessage {
    pub custom: Arc<CustomSpread>,
}

impl Reducer<DataState> for SetCustomSpreadMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();
        state.custom_spread = Some(self.custom);
        state.into()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct BackToTopMessage;

impl Reducer<DataState> for BackToTopMessage {
    fn apply(self, state: Rc<DataState>) -> Rc<DataState> {
        let mut state = (*state).clone();
        state.back_to_top();
        state.into()
    }
}
