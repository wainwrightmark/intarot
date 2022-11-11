

use std::{collections::BTreeMap, io::BufRead, str::FromStr};




use crate::data::prelude::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Default)]
pub struct ImageMetaState {
    pub metas: Option<BTreeMap<(StarSign, Soothsayer, Card), ImageMeta>>, //TODO bTreeMap
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
        let result: BTreeMap<_, ImageMeta> = lines
            .skip(1) //skip headers
            .filter_map(|x| x.ok())
            .map(move |x| ImageMeta::from_str(x.as_str()).unwrap())
            .map(|x| ((x.sign, x.soothsayer, x.card), x))
            .collect();
        Ok(ImageMetaState {
            metas: result.into(),
        })
    }
}
