use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{prelude::Card, guide::Guide};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub id: Arc<String>,
    pub image_type: ImageType,
    pub alt: String
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ImageType {
    CardPlaceholder,
    Card,
    Final,
    Guide,
    Custom,
}

impl ImageData {

    pub fn placeholder(guide: Guide, card: Card)-> Self{
        let g = guide.short_name();
            let c = (b'a' + (card as u8)) as char;
            let id = format!("{g}{c}AA");
        Self { id:id.into(), image_type: ImageType::CardPlaceholder, alt: card.name().to_string() }
    }

    /// Img src - used for each card
    pub fn src(&self) -> String {
        let id = self.id.clone();
        match self.image_type {
            ImageType::Card => format!(
                "https://intarot-images.s3.eu-west-2.amazonaws.com/Upscaled Images/{id}.jpg"
            ),
            ImageType::Final => {
                format!("https://intarot-images.s3.eu-west-2.amazonaws.com/AdCards/{id}.jpg")
            },
            ImageType::Guide => {
                format!("https://intarot-images.s3.eu-west-2.amazonaws.com/Soothsayers/{id}.jpg")
            },
            ImageType::Custom => {
                format!("https://intarot-images.s3.eu-west-2.amazonaws.com/Custom/{id}.jpg")
            },
            ImageType::CardPlaceholder=>{
                format!("/images/blank/{id}.png")
            }
        }
    }

    pub fn alt(&self)-> String{
        self.alt.clone()
    }
}
