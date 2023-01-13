use crate::data::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct DrawMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ReplaceMessage {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MaybeChangeDataMessage(pub QuestionData);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ResetMessage;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleDescriptionMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleShareDialogMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct SetUsedBeforeMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ShufflePromptsMessage;
