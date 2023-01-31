use serde::{Deserialize, Serialize};

use super::{prelude::*, spread_id::SpreadId};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum SrcData {
    Card(&'static str),
    Spread{
        card_name: &'static str,
        question_data: QuestionData,
        perm: Perm
    },
    Guide(&'static str),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SrcData2 {
    Card(String),
    Spread{
        card_name: String,
        question_data: QuestionData,
        perm: Perm
    },
    Guide(String),
}

impl From<SrcData> for SrcData2 {
    fn from(value: SrcData) -> Self {
        match value {
            SrcData::Card(s) => Self::Card(s.to_string()),
            SrcData::Guide(s) => Self::Guide(s.to_string()),
            SrcData::Spread { card_name, question_data, perm } => Self::Spread { card_name: card_name.to_string(), question_data, perm },

        }
    }
}

impl SrcData {
    /// Img src - used for each card
    pub fn src(&self) -> String {
        match self {
            SrcData::Card(name) => format!(
                "https://intarot-images.s3.eu-west-2.amazonaws.com/Upscaled Images/{name}.jpg"
            ),
            SrcData::Guide(name) => {
                format!("https://intarot-images.s3.eu-west-2.amazonaws.com/Soothsayers/{name}.jpg")
            },
            SrcData::Spread { question_data, .. } => format!(

                "https://intarot-images.s3.eu-west-2.amazonaws.com/AdCards/{}.jpg",
                question_data.guide.ad_image_src()
            ),
        }
    }

    pub fn share_url(&self) -> String {
        match self {
            SrcData::Card(name) => format!("https://intarot.app/share?id={name}"),
            SrcData::Guide(_) => "https://intarot.app".to_string(),
            SrcData::Spread { card_name, question_data, perm } => {
                let spread_id = SpreadId::new(question_data, perm);
                let spread_id_encoded = spread_id.encode();
                format!("https://intarot.app/share?id={card_name}&spread={spread_id_encoded}")
            },
        }
    }

    pub fn url_has_search(&self) -> bool {
        match self {
            SrcData::Card(_) => true,
            SrcData::Spread{..} => true,
            SrcData::Guide(_) => false,
        }
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
