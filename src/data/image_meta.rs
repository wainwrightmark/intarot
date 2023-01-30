use std::str::FromStr;

use strum::IntoEnumIterator;
use num_traits::cast::FromPrimitive;

use crate::{data::prelude::*};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ImageMeta {
    pub file_name: &'static str,
    pub guide: Guide,
    pub card: Card,
}

impl FromStr for ImageMeta {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_name = Box::leak(s.to_string().into_boxed_str());

        let guide = Guide::iter()
            .find(|g| file_name.starts_with(g.short_name()))
            .ok_or_else(|| anyhow::anyhow!("Could not find guide for {file_name}"))?;

        let char_c = file_name.chars().skip(1).next().ok_or_else(|| anyhow::anyhow!("Could not find card for {file_name}"))? as u8;
        let char_c = char_c - b'a';
        let card = Card::from_u8(char_c).ok_or_else(|| anyhow::anyhow!("Could not find card for {file_name}"))?;

        Ok(ImageMeta {
            file_name,
            guide,
            card,
        })
    }
}

impl ImageMeta {
    pub fn src_data(&self) -> SrcData {
        SrcData::Card(self.file_name)
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
