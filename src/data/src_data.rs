use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum SrcData {
    Card(&'static str),
    Ad(&'static str),
    Guide(&'static str),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SrcData2 {
    Card(String),
    Ad(String),
    Guide(String),
}

impl From<SrcData> for SrcData2 {
    fn from(value: SrcData) -> Self {
        match value {
            SrcData::Card(s) => Self::Card(s.to_string()),
            SrcData::Ad(s) => Self::Ad(s.to_string()),
            SrcData::Guide(s) => Self::Guide(s.to_string()),
        }
    }
}

impl SrcData {
    pub fn src(&self) -> String {
        match self {
            SrcData::Card(name) => format!(
                "https://intarot-images.s3.eu-west-2.amazonaws.com/Upscaled Images/{name}.jpg"
            ),
            SrcData::Ad(name) => {
                format!("https://intarot-images.s3.eu-west-2.amazonaws.com/AdCards/{name}.jpg")
            }
            SrcData::Guide(name) => {
                format!("https://intarot-images.s3.eu-west-2.amazonaws.com/Soothsayers/{name}.jpg")
            }
        }
    }

    pub fn share_url(&self) -> String {
        match self {
            SrcData::Card(name) => format!("https://intarot.app/share?id={}", { name }),
            SrcData::Ad(_) | SrcData::Guide(_) => "https://intarot.app".to_string(),
        }
    }

    pub fn url_has_search(&self) -> bool {
        match self {
            SrcData::Card(_) => true,
            SrcData::Ad(_) | SrcData::Guide(_) => false,
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
