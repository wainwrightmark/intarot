use std::{collections::HashMap, str::FromStr};

use crate::data::prelude::*;
use itertools::Itertools;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store)]
pub struct ImageMetaState {
    pub metas: HashMap<MetaKey, Vec<ImageMeta>>,
}

impl Default for ImageMetaState {
    fn default() -> Self {
        let data = include_str!("../tsv/image_names.tsv");
        let lines = data.lines();

        let metas = lines
            .map(move |str| ImageMeta::from_str(str).unwrap())
            .into_group_map_by(|image_meta| MetaKey::from(image_meta.clone()));
        Self { metas }
    }
}
