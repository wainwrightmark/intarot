use std::{collections::BTreeMap,str::FromStr};

use crate::data::prelude::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store)]
pub struct ImageDescriptionState {
    pub descriptions: BTreeMap<(Guide, Card), ImageDescription>,
}

impl Default for ImageDescriptionState{
    fn default() -> Self {
        let data = include_str!("..\\tsv\\descriptions.tsv");
        let lines = data.lines();
        let descriptions: BTreeMap<_, _> = lines
            .skip(1) //skip headers
            //.filter_map(|x| x.ok())
            .filter_map(move |x| ImageDescription::from_str(x).ok())
            .map(|x| ((x.guide, x.card), x))
            .collect();

        Self { descriptions }
    }
}
