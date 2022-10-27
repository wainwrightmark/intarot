use crate::web::prelude::*;

use itertools::Itertools;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct ImageState {
    pub sign: Option<StarSign>,
    pub soothsayer: Option<Soothsayer>,
    pub seed: u64,
}

impl ImageState {

    pub fn reroll(&mut self){
        self.seed += 1;
    }

    pub fn get_image_meta(&self) -> Option<ImageMeta> {
        if let Some(sign) = self.sign {
            if let Some(soothsayer) = self.soothsayer {
                let mut seeded_rng = StdRng::seed_from_u64(self.seed);

                let metas = IMAGEMETAS
                    .iter()
                    .filter(|x| x.sign == sign && x.soothsayer == soothsayer)
                    .cloned()
                    .collect_vec();

                if metas.is_empty() {
                    return None;
                }

                let index = seeded_rng.gen_range(0..metas.len());

                let meta = metas.get(index).map(|x|x.clone());

                return meta;
            }
        }

        None
    }
}
