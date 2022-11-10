use crate::web::prelude::*;
use std::{str::FromStr, io::BufRead, rc::Rc};
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

#[derive(PartialEq, Eq, Store, Default)]
pub struct ImageMetaState{
    pub metas: Option<Vec<ImageMeta>>
}

impl ImageMetaState{
pub async fn setup(){
    let result = Self::create().await.unwrap();    
    Dispatch::<ImageMetaState>::new().set(result);
}

    pub async fn create() -> Result<Self, anyhow::Error>{
        let url = "https://docs.google.com/spreadsheets/d/e/2PACX-1vTdGJ64J-Iivs6MuSlXyuemE56GsYNqDlTGb3hohHtVl3xq6XuzxYtMrU5AL8CCjGDwhW_lEiRoXoFA/pub?gid=0&single=true&output=csv";
        let result = reqwest::get(url).await;
        let data =result?;
        let bytes = data.bytes().await?;
        let lines = bytes.lines();
        let result = lines
        .skip(1)//skip headers        
        .filter_map(|x|x.ok())
        .map(move|x|ImageMeta::from_str(x.as_str())
        .unwrap())
        .collect_vec();
        Ok(ImageMetaState{metas: result.into()})
    }
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

    pub fn can_next_soothsayer(&self) -> bool {
        match self {
            ImageState::OpeningPage(_) => false,
            ImageState::SoothsayerPage(_, soothsayer) => soothsayer.next().is_some(),
            ImageState::CardPage(_, _, _) => false,
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

    pub fn can_previous_soothsayer(&self) -> bool {
        match self {
            ImageState::OpeningPage(_) => false,
            ImageState::SoothsayerPage(_, soothsayer) => soothsayer.previous().is_some(),
            ImageState::CardPage(_, _, _) => false,
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

    pub fn can_proceed(&self) -> bool {
        match self {
            ImageState::OpeningPage(ss) => ss.is_some(),
            ImageState::SoothsayerPage(_ss, _soothsayer) => true,
            ImageState::CardPage(_, _, _) => false,
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
            
                let meta_state = Dispatch::<ImageMetaState>::new().get();
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
