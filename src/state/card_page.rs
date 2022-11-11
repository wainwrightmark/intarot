use itertools::Itertools;

use rand::Rng;
use strum::EnumCount;
use strum::IntoEnumIterator;

use crate::data::prelude::Card;
use crate::data::prelude::ImageMeta;
use crate::data::prelude::{Ordering, Soothsayer, StarSign};

use super::soothsayer_page::SoothsayerPage;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Default)]
pub struct CardPage {
    pub star_sign: StarSign,
    pub soothsayer: Soothsayer,
    pub ordering: Ordering,
    pub cards_drawn: usize,
    pub show_description: bool,
}

impl From<&SoothsayerPage> for CardPage {
    fn from(value: &SoothsayerPage) -> Self {
        let mut rng = rand::thread_rng();

        let ordering = rng.gen_range(Ordering::get_range(&Card::COUNT));
        CardPage {
            star_sign: value.star_sign,
            soothsayer: value.soothsayer,
            ordering: ordering.into(),
            cards_drawn: 1,
            show_description: false,
        }
    }
}

impl CardPage {
    pub fn draw_card(mut self) -> Self {
        if self.cards_drawn < Card::COUNT {
            self.cards_drawn += 1;
        }
        self.show_description = false;
        self
    }

    pub fn shuffle(mut self) -> Self {
        let mut rng = rand::thread_rng();
        let ordering = rng.gen_range(Ordering::get_range(&Card::COUNT));

        self.cards_drawn = 1;
        self.ordering = ordering.into();
        self
    }

    pub fn toggle_description(mut self) -> Self {
        self.show_description = !self.show_description;
        self
    }

    pub fn get_possible_image_metas(&self, meta_state: &super::prelude::ImageMetaState) -> Vec<ImageMeta> {
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

//StarSign, Soothsayer, u64, bool
