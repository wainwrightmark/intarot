use itertools::Itertools;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::data::prelude::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub enum PageState {
    OpeningPage(Option<StarSign>),
    SoothsayerPage(StarSign, Soothsayer),
    CardPage(StarSign, Soothsayer, u64, bool),
}

impl Default for PageState {
    fn default() -> Self {
        Self::OpeningPage(None)
    }
}

impl PageState {
    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn reroll(&mut self) {
        match self {
            PageState::OpeningPage(_) => todo!(),
            PageState::SoothsayerPage(_, _) => todo!(),
            PageState::CardPage(_, _, s, d) => {
                *s += 1;
                *d = false;
            }
        }
    }

    pub fn get_soothsayer(&self) -> Soothsayer {
        match self {
            PageState::OpeningPage(_) => Soothsayer::first(),
            PageState::SoothsayerPage(_, sayer) => *sayer,
            PageState::CardPage(_, sayer, _, _) => *sayer,
        }
    }

    pub fn get_sign(&self) -> Option<StarSign> {
        match self {
            PageState::OpeningPage(ss) => *ss,
            PageState::SoothsayerPage(ss, _) => Some(*ss),
            PageState::CardPage(ss, _, _, _) => Some(*ss),
        }
    }

    pub fn can_next_soothsayer(&self) -> bool {
        match self {
            PageState::OpeningPage(_) => false,
            PageState::SoothsayerPage(_, soothsayer) => soothsayer.next().is_some(),
            PageState::CardPage(_, _, _, _) => false,
        }
    }

    pub fn next_soothsayer(&mut self) {
        match self {
            PageState::OpeningPage(_) => {}
            PageState::SoothsayerPage(_, soothsayer) => {
                if let Some(next) = soothsayer.next() {
                    *soothsayer = next;
                }
            }
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn can_previous_soothsayer(&self) -> bool {
        match self {
            PageState::OpeningPage(_) => false,
            PageState::SoothsayerPage(_, soothsayer) => soothsayer.previous().is_some(),
            PageState::CardPage(_, _, _, _) => false,
        }
    }

    pub fn previous_soothsayer(&mut self) {
        match self {
            PageState::OpeningPage(_) => {}
            PageState::SoothsayerPage(_, soothsayer) => {
                if let Some(prev) = soothsayer.previous() {
                    *soothsayer = prev;
                }
            }
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn set_star_sign(&mut self, new_star_sign: Option<StarSign>) {
        match self {
            PageState::OpeningPage(ss) => *ss = new_star_sign,
            PageState::SoothsayerPage(_, _) => {}
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn can_proceed(&self) -> bool {
        match self {
            PageState::OpeningPage(ss) => ss.is_some(),
            PageState::SoothsayerPage(_ss, _soothsayer) => true,
            PageState::CardPage(_, _, _, _) => false,
        }
    }

    pub fn proceed(&mut self) {
        match self {
            PageState::OpeningPage(ss) => {
                if let Some(star) = ss {
                    *self = Self::SoothsayerPage(*star, Soothsayer::first())
                }
            }
            PageState::SoothsayerPage(ss, soothsayer) => {
                *self = Self::CardPage(*ss, *soothsayer, 1, false)
            }
            PageState::CardPage(_, _, _, _) => {}
        }
    }

    pub fn show_description(&self) -> bool {
        match self {
            PageState::OpeningPage(_) => false,
            PageState::SoothsayerPage(_, _) => false,
            PageState::CardPage(_, _, _, s) => *s,
        }
    }

    pub fn toggle_show_description(&mut self) {
        match self {
            PageState::OpeningPage(_) => (),
            PageState::SoothsayerPage(_, _) => (),
            PageState::CardPage(_, _, _, s) => *s = !(*s),
        }
    }

    pub fn get_image_meta(&self, meta_state: &super::prelude::ImageMetaState) -> Option<ImageMeta> {
        match self {
            PageState::OpeningPage(_) => None,
            PageState::SoothsayerPage(_, _) => None,
            PageState::CardPage(sign, soothsayer, seed, _) => {
                let mut seeded_rng = StdRng::seed_from_u64(*seed);

                //let meta_state = Dispatch::<ImageMetaState>::new().get();
                let Some(all_metas) = meta_state.metas.as_ref()
                else{
                    return None;
                };
                let metas = all_metas
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
