use std::str::FromStr;

use strum::IntoEnumIterator;

use crate::data::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ImageMeta {
    pub file_name: &'static str,
    pub star_sign: StarSign,
    pub guide: Guide,
    pub card: Card,
}

impl FromStr for ImageMeta {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_name = Box::leak(s.to_string().into_boxed_str());

        let guide = Guide::iter()
            .find(|ss| ss.filter_image(file_name))
            .unwrap_or_else(|| panic!("Could not find guide for {file_name}"));

        let sign = StarSign::iter()
            .find(|ss| ss.filter_image(file_name))
            .unwrap_or_else(|| panic!("Could not find sign for {file_name}"));

        let card = Card::iter()
            .find(|ss| ss.filter_image(file_name))
            .unwrap_or_else(|| panic!("Could not find card for {file_name}"));

        Ok(ImageMeta {
            file_name,
            star_sign: sign,
            guide,
            card,
        })
    }
}

impl ImageMeta{
    pub fn src(&self)-> String{
        format!("https://intarot-images.s3.eu-west-2.amazonaws.com/Upscaled Images/{}.jpg",self.file_name)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
pub struct MetaKey {
    pub star_sign: Option<StarSign>,
    pub guide: Guide,
    pub card: Card,
}

impl MetaKey {
    pub fn with_no_sign(&self) -> Self {
        Self {
            star_sign: None,
            guide: self.guide,
            card: self.card,
        }
    }
}

impl From<ImageMeta> for MetaKey {
    fn from(value: ImageMeta) -> Self {
        Self {
            star_sign: Some(value.star_sign),
            guide: value.guide,
            card: value.card,
        }
    }
}
