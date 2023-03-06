use std::str::FromStr;

use num_traits::cast::FromPrimitive;
use strum::IntoEnumIterator;

use crate::data::prelude::*;

use super::image_data::ImageData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImageMeta {
    pub image_data: ImageData,
    pub guide: Guide,
    pub card: Card,
}

impl FromStr for ImageMeta {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s; // Box::leak(s.to_string().into_boxed_str());

        let guide = Guide::iter()
            .find(|g| id.starts_with(g.short_name()))
            .ok_or_else(|| anyhow::anyhow!("Could not find guide for {id}"))?;

        let char_c =
            id.chars()
                .nth(1)
                .ok_or_else(|| anyhow::anyhow!("Could not find card for {id}"))? as u8;
        let char_c = char_c - b'a';
        let card =
            Card::from_u8(char_c).ok_or_else(|| anyhow::anyhow!("Could not find card for {id}"))?;

        Ok(ImageMeta {
            image_data: ImageData {
                id: id.to_string().into(),
                image_type: super::image_data::ImageType::Card,
                alt:card.name().to_string().into()
            },
            guide,
            card,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
pub struct MetaKey {
    pub guide: Guide,
    pub card: Card,
}

impl From<ImageMeta> for MetaKey {
    fn from(value: ImageMeta) -> Self {
        Self {
            guide: value.guide,
            card: value.card,
        }
    }
}
