use std::rc::Rc;

use itertools::Itertools;

use rand::RngCore;
use rand::rngs::StdRng;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::SeedableRng;
// use rand::Rng;
use strum::EnumCount;
use strum::IntoEnumIterator;
use yewdux::store::Store;

use crate::data::prelude::Card;
use crate::data::prelude::ImageMeta;
use crate::data::prelude::{ Soothsayer, StarSign};

#[derive(PartialEq, Eq, Clone, Copy, serde:: Serialize, serde::Deserialize, Store)]
pub struct CardPageState {
    pub star_sign: Option<StarSign>,
    pub soothsayer: Soothsayer,
    pub seed: u32,
    pub cards_drawn: usize,
    pub show_description: bool,
    pub max_drawn: usize,
    pub has_shown_description: bool
    // pub share_dialog_open: bool,
}

impl Default for CardPageState {
    fn default() -> Self {
        let seed = ThreadRng::default().next_u32();
        Self {
            cards_drawn: 1,
            max_drawn: 1,
            seed,
            star_sign: Default::default(),
            soothsayer: Default::default(),
            show_description: false,
            has_shown_description: false
            // share_dialog_open: false,
        }
    }
}

impl CardPageState {
    pub fn get_new_seed_if_changed(
        &self,
        star_sign: Option<StarSign>,
        soothsayer: Soothsayer,
    ) -> u32 {
        if self.star_sign == star_sign && self.soothsayer == soothsayer {
            self.seed
        } else {
            ThreadRng::default().next_u32()
        }
    }

    pub fn on_load(
        self: Rc<Self>,
        new_sign: Option<StarSign>,
        new_soothsayer: Soothsayer,
        new_seed: u32,
    ) -> Rc<Self> {
        if new_sign != self.star_sign
            || new_soothsayer != self.soothsayer
            || new_seed != self.seed
        {
            Self {
                star_sign: new_sign,
                soothsayer: new_soothsayer,
                seed: new_seed,
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
        self.has_shown_description = true;
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
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        cards.shuffle(&mut rng);
        let star_sign = self.star_sign.unwrap_or_else(|| StarSign::iter().choose(&mut rng).unwrap());

        cards
            .into_iter()
            .flat_map(|card| all_metas.get(&(star_sign, self.soothsayer, card)))
            .cloned()
            .collect_vec()
    }
}
