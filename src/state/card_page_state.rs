use std::rc::Rc;

use itertools::Itertools;

// use rand::Rng;
use strum::EnumCount;
use strum::IntoEnumIterator;
use yewdux::store::Store;

use crate::data::prelude::Card;
use crate::data::prelude::ImageMeta;
use crate::data::prelude::{Ordering, Soothsayer, StarSign};

#[derive(PartialEq, Eq, Clone, Copy, serde:: Serialize, serde::Deserialize, Store)]
pub struct CardPageState {
    pub star_sign: StarSign,
    pub soothsayer: Soothsayer,
    pub ordering: Ordering,
    pub cards_drawn: usize,
    pub show_description: bool,
    pub max_drawn: usize,
    // pub share_dialog_open: bool,
}

impl Default for CardPageState {
    fn default() -> Self {
        let ordering = Ordering::gen(22);
        Self {
            cards_drawn: 1,
            max_drawn: 1,
            ordering,
            star_sign: Default::default(),
            soothsayer: Default::default(),
            show_description: false,
            // share_dialog_open: false,
        }
    }
}

impl CardPageState {
    pub fn get_new_ordering_if_changed(
        &self,
        star_sign: StarSign,
        soothsayer: Soothsayer,
    ) -> Ordering {
        if self.star_sign == star_sign && self.soothsayer == soothsayer {
            self.ordering
        } else {
            Ordering::gen(22)
        }
    }

    pub fn on_load(
        self: Rc<Self>,
        new_sign: StarSign,
        new_soothsayer: Soothsayer,
        new_ordering: Ordering,
    ) -> Rc<Self> {
        if new_sign != self.star_sign
            || new_soothsayer != self.soothsayer
            || new_ordering != self.ordering
        {
            Self {
                star_sign: new_sign,
                soothsayer: new_soothsayer,
                ordering: new_ordering,
                ..Default::default()
            }
            .into()
        } else {
            self
        }
    }

    pub fn draw_card(mut self) -> Self {
        self.show_description = false;
        if self.cards_drawn <= Card::COUNT {
            self.cards_drawn += 1;
            self.max_drawn = self.max_drawn.max(self.cards_drawn + 1);

            self
        } else {
            self
            // self.shuffle()
        }
    }

    pub fn replace_card(mut self) -> Self {
        if self.cards_drawn > 1 {
            self.cards_drawn -= 1;
        }
        self.show_description = false;
        self
    }

    // pub fn shuffle(mut self) -> Self {
    //     let mut rng = rand::thread_rng();
    //     let ordering = rng.gen_range(Ordering::get_range(&Card::COUNT));

    //     self.cards_drawn = 0;
    //     self.ordering = ordering.into();
    //     self
    // }

    pub fn toggle_description(mut self) -> Self {
        self.show_description = !self.show_description;
        self
    }

    // pub fn toggle_dialog_open(mut self) -> Self {
    //     self.share_dialog_open = !self.share_dialog_open;
    //     self
    // }

    pub fn get_possible_image_metas(
        &self,

        meta_state: &super::prelude::ImageMetaState,
    ) -> Vec<ImageMeta> {
        let Some(all_metas) = meta_state.metas.as_ref()
                else{
                    return Default::default();
                };

        let mut cards = Card::iter().collect_vec();

        self.ordering.reorder(&mut cards);

        cards
            .into_iter()
            .flat_map(|card| all_metas.get(&(self.star_sign, self.soothsayer, card)))
            .cloned()
            .collect_vec()
    }
}
