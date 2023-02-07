use serde::{Deserialize, Serialize};

use super::{
    image_data::{ImageData, ImageType},
    prelude::*,
    spread_id::SpreadId,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SrcData {
    pub image: ImageData,
    pub spread_option: Option<SpreadShare>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SpreadShare {
    pub question_data: QuestionData,
    pub perm: Perm,
    pub share_img: ImageData,
}

impl SrcData {
    pub fn share_url(&self) -> String {
        match self.image.image_type {
            ImageType::Card => format!("https://intarot.app/share?id={}", self.image.id.clone()),
            ImageType::Final => match &self.spread_option {
                Some(SpreadShare {
                    question_data,
                    perm,
                    share_img,
                }) => {
                    let spread_id = SpreadId::new(question_data, perm);
                    let spread_id_encoded = spread_id.encode();
                    let id = share_img.id.clone();
                    format!("https://intarot.app/share?id={id}&spread={spread_id_encoded}")
                }
                None => "https://intarot.app".to_string(),
            },
            _ => "https://intarot.app".to_string(),
        }
    }

    pub fn url_has_search(&self) -> bool {
        matches!(self.image.image_type, ImageType::Card)
    }

    pub fn raw_url_with_ref(&self, referrer: &'static str) -> String {
        let url = self.share_url();
        let separator = if self.url_has_search() { "&" } else { "?" };
        format!("{url}{separator}ref={referrer}")
    }

    pub fn encoded_url_with_ref(&self, referrer: &'static str) -> String {
        url_escape::encode_www_form_urlencoded(self.raw_url_with_ref(referrer).as_str()).to_string()
    }
}
