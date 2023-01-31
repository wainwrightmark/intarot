use crate::data::prelude::*;

use super::prelude::LoggableEvent;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct DrawMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ReplaceMessage {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ChangeGuideMessage(pub Guide);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ChangeSpreadTypeMessage(pub SpreadType);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LoadSpreadMessage(pub QuestionData, pub Perm);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ResetMessage;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleDescriptionMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleShareDialogMessage {}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ShufflePromptsMessage;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct CreateUserIfNewMessage {
    pub referrer: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct AchievementEarnedMessage(pub Achievement);


#[derive(Clone, PartialEq, Eq)]
pub struct LogFailedMessage(pub LoggableEvent);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ResentFailedLogsMessage;
