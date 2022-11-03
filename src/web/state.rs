use std::default;

use crate::web::prelude::*;

use itertools::Itertools;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub enum ImageState {
    OpeningPage(Option<StarSign>),
    SoothsayerPage(StarSign, Soothsayer),
    CardPage(StarSign, Soothsayer, u64),
}

impl Default for ImageState {
    fn default() -> Self {
        Self::OpeningPage(None)
    }
}

impl ImageState {
    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn reroll(&mut self) {
        match self {
            ImageState::OpeningPage(_) => todo!(),
            ImageState::SoothsayerPage(_, _) => todo!(),
            ImageState::CardPage(_, _, s) => *s += 1,
        }
    }

    pub fn get_soothsayer(&self) -> Soothsayer {
        match self {
            ImageState::OpeningPage(_) => Soothsayer::first(),
            ImageState::SoothsayerPage(_, sayer) => *sayer,
            ImageState::CardPage(_, sayer, _) => *sayer,
        }
    }

    pub fn get_sign(&self) -> Option<StarSign> {
        match self {
            ImageState::OpeningPage(ss) => *ss,
            ImageState::SoothsayerPage(ss, _) => Some(*ss),
            ImageState::CardPage(ss, _, _) => Some(*ss),
        }
    }

    pub fn can_next_soothsayer(&self)-> bool{
        match self {
            ImageState::OpeningPage(_) => false,
            ImageState::SoothsayerPage(_, soothsayer) => {
                soothsayer.next().is_some()
            }
            ImageState::CardPage(_, _, _) => false
        }
    }

    pub fn next_soothsayer(&mut self) {
        match self {
            ImageState::OpeningPage(_) => {}
            ImageState::SoothsayerPage(_, soothsayer) => {
                if let Some(next) = soothsayer.next() {
                    *soothsayer = next;
                }
            }
            ImageState::CardPage(_, _, _) => {}
        }
    }

    pub fn can_previous_soothsayer(&self)-> bool{
        match self {
            ImageState::OpeningPage(_) => false,
            ImageState::SoothsayerPage(_, soothsayer) => {
                soothsayer.previous().is_some()
            }
            ImageState::CardPage(_, _, _) => false
        }
    }

    pub fn previous_soothsayer(&mut self) {
        match self {
            ImageState::OpeningPage(_) => {}
            ImageState::SoothsayerPage(_, soothsayer) => {
                if let Some(prev) = soothsayer.previous() {
                    *soothsayer = prev;
                }
            }
            ImageState::CardPage(_, _, _) => {}
        }
    }

    pub fn set_star_sign(&mut self, new_star_sign: Option<StarSign>) {
        match self {
            ImageState::OpeningPage(ss) => *ss = new_star_sign,
            ImageState::SoothsayerPage(_, _) => {}
            ImageState::CardPage(_, _, _) => {}
        }
    }

    pub fn can_proceed(&self)-> bool{
        match self {
            ImageState::OpeningPage(ss) => ss.is_some(),
            ImageState::SoothsayerPage(ss, soothsayer) => true,
            ImageState::CardPage(_, _, _) => {false}
        }
    }

    pub fn proceed(&mut self) {
        match self {
            ImageState::OpeningPage(ss) => {
                if let Some(star) = ss {
                    *self = Self::SoothsayerPage(*star, Soothsayer::first())
                }
            }
            ImageState::SoothsayerPage(ss, soothsayer) => {
                *self = Self::CardPage(*ss, *soothsayer, 1)
            }
            ImageState::CardPage(_, _, _) => {}
        }
    }

    pub fn get_image_meta(&self) -> Option<ImageMeta> {
        match self {
            ImageState::OpeningPage(_) => None,
            ImageState::SoothsayerPage(_, _) => None,
            ImageState::CardPage(sign, soothsayer, seed) => {
                let mut seeded_rng = StdRng::seed_from_u64(*seed);

                let metas = IMAGEMETAS
                    .iter()
                    .filter(|x| x.sign == *sign && x.soothsayer == *soothsayer)
                    .cloned()
                    .collect_vec();

                if metas.is_empty() {
                    return None;
                }

                let index = seeded_rng.gen_range(0..metas.len());

                let meta = metas.get(index).map(|x| x.clone());

                meta
            }
        }
    }
}
