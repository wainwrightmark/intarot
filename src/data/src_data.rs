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
            SrcData::Card(name) => format!("https://intarot.com/share?id={}", {
                // log::info!("{encoded}");
                Engine::encode(&base64::engine::general_purpose::URL_SAFE, name)
            }),
            _ => "https://intarot.com".to_string(),
        }
    }
}
