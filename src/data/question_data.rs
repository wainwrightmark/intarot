use serde::{Deserialize, Serialize};
use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionData {
    pub guide: Guide,
    pub spread_type: SpreadType,
}

impl Default for QuestionData{
    fn default() -> Self {
        Self { guide: Default::default(), spread_type: SpreadType::Three }
    }
}
