use std::{collections::BTreeMap, str::FromStr};

use crate::data::prelude::*;

use chrono::Datelike;
use itertools::Itertools;
use rand::{seq::SliceRandom, SeedableRng};
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store)]
pub struct ImageMetaState {
    pub metas: BTreeMap<MetaKey, ImageMeta>,
}

impl Default for ImageMetaState {
    fn default() -> Self {
        let data = include_str!("../tsv/image_names.tsv");
        let lines = data.lines();

        let mut metas_vec = lines
            .map(move |x| ImageMeta::from_str(x).unwrap())
            .map(|x| (MetaKey::from(x), x))
            .collect_vec();

        let today = get_today_date();
        let seed = (today.year().unsigned_abs() * 2000) + (today.month() * 100) + today.day();
        let mut rng: rand::rngs::StdRng = SeedableRng::seed_from_u64(seed as u64);

        metas_vec.shuffle(&mut rng);

        let metas: BTreeMap<_, ImageMeta> = metas_vec.into_iter().collect();
        Self { metas }
    }
}

fn get_today_date() -> chrono::NaiveDate {
    let today = chrono::offset::Utc::now();
    today.date_naive()
}
