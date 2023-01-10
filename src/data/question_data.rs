use serde::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct QuestionData {
    pub star_sign: Option<StarSign>,
    pub guide: Guide,
    pub spread_type: SpreadType,
}
