use std::str::FromStr;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::data::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct ImageMeta {
    pub id: String,
    pub sign: StarSign,
    pub guide: Guide,
    pub card: Card,
}

impl FromStr for ImageMeta {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, file_name) = s.split_terminator('\t').next_tuple().unwrap();

        let guide = Guide::iter()
            .find(|ss| ss.filter_image(file_name))
            .unwrap_or_else(|| panic!("Could not find guide for {}", file_name));

        let sign = StarSign::iter()
            .find(|ss| ss.filter_image(file_name))
            .unwrap_or_else(|| panic!("Could not find sign for {}", file_name));

        let card = Card::iter()
            .find(|ss| ss.filter_image(file_name))
            .unwrap_or_else(|| panic!("Could not find card for {}", file_name));

        Ok(ImageMeta {
            id: id.to_string(),
            sign,
            guide,
            card,
        })
    }
}
