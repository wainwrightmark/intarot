use crate::data::prelude::StarSign;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Default)]
pub struct OpeningPage {
    pub star_sign: Option<StarSign>,
}
