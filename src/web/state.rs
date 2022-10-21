use crate::web::prelude::*;

use itertools::Itertools;

use serde::{Deserialize, Serialize};

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct ImageState {
    pub sign: Option<StarSign>,
    pub soothsayer: Option<Soothsayer>,
}

impl ImageState {
    pub fn get_image_meta(&self) -> Option<Vec<ImageMeta>> {
        if let Some(sign) = self.sign {
            if let Some(soothsayer) = self.soothsayer {
                let metas = IMAGEMETAS
                    .iter()
                    .filter(|x| x.sign == sign && x.soothsayer == soothsayer)
                    .cloned()
                    .collect_vec();

                return Some(metas);
            }
        }

        None
    }
}
