use base64::Engine;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SrcData {
    Card(&'static str),
    Ad(&'static str),
    Guide(&'static str),
}

impl SrcData {
    pub fn src(&self) -> String {
        match self {
            SrcData::Card(name) => format!(
                "https://intarot-images.s3.eu-west-2.amazonaws.com/Upscaled Images/{}.jpg",
                name
            ),
            SrcData::Ad(name) => format!(
                "https://intarot-images.s3.eu-west-2.amazonaws.com/AdCards/{}.jpg",
                name
            ),
            SrcData::Guide(name) => format!(
                "https://intarot-images.s3.eu-west-2.amazonaws.com/Soothsayers/{}.jpg",
                name
            ),
        }
    }

    pub fn share_url(&self) -> String {
        match self {
            SrcData::Card(name) => format!(
                "https://www.intarot.com/#/share/{}",
                {
                    let encoded = Engine::encode(&base64::engine::general_purpose::URL_SAFE, name);
                    log::info!("{encoded}");
                    encoded
                }
            ),
            _ => "https://www.intarot.com".to_string(),
        }
    }
}
