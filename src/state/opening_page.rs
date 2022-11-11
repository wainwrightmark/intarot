use yewdux::store::Reducer;

use crate::data::prelude::StarSign;

use super::prelude::PageState;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Default)]
pub struct OpeningPage {
    pub star_sign: Option<StarSign>,
}
