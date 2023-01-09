use std::{collections::BTreeMap, io::BufRead, str::FromStr};

use crate::data::prelude::*;

use chrono::Datelike;
use itertools::Itertools;
use rand::{seq::SliceRandom, SeedableRng};
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Default)]
pub struct ImageMetaState {
    pub metas: Option<BTreeMap<(StarSign, Guide, Card), ImageMeta>>, //TODO bTreeMap
}

impl ImageMetaState {
    pub async fn setup() {
        let result = Self::create().await.unwrap();
        Dispatch::<ImageMetaState>::new().set(result);
    }

    pub async fn create() -> Result<Self, anyhow::Error> {
        let url = "https://docs.google.com/spreadsheets/d/e/2PACX-1vTdGJ64J-Iivs6MuSlXyuemE56GsYNqDlTGb3hohHtVl3xq6XuzxYtMrU5AL8CCjGDwhW_lEiRoXoFA/pub?gid=0&single=true&output=tsv";
        let result = reqwest::get(url).await;
        let data = result?;
        let bytes = data.bytes().await?;
        let lines = bytes.lines();

        let mut metas_vec = lines
            .skip(1) //skip headers
            .filter_map(|x| x.ok())
            .map(move |x| ImageMeta::from_str(x.as_str()).unwrap())
            .map(|x| ((x.sign, x.guide, x.card), x))
            .collect_vec();

        let today = get_today_date();
        let seed = ((today.year().abs() as u32) * 2000)
                + (today.month() * 100)
                + today.day();
        let mut rng: rand::rngs::StdRng = SeedableRng::seed_from_u64(seed as u64);

        metas_vec.shuffle(&mut rng);

        let result: BTreeMap<_, ImageMeta> = metas_vec.into_iter().collect();
        Ok(ImageMetaState {
            metas: result.into(),
        })
    }
}


fn get_today_date() -> chrono::NaiveDate {
    let today = chrono::offset::Utc::now();
    today.date_naive()
}