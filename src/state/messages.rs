use crate::data::prelude::*;

use super::prelude::LoggableEvent;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LoadSpreadMessage(pub QuestionData, pub Perm);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleShareDialogMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ShufflePromptsMessage;

#[derive(Clone, PartialEq, Eq)]
pub struct AchievementEarnedMessage(pub Achievement);

#[derive(Clone, PartialEq, Eq)]
pub struct LogFailedMessage(pub LoggableEvent);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ResentFailedLogsMessage;
