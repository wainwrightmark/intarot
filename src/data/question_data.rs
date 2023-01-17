use serde::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct QuestionData {
    pub guide: Guide,
    pub spread_type: SpreadType,
}
