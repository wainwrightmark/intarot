use std::ops::Not;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::EnumDiscriminants;
use uuid::Uuid;
use yewdux::prelude::Dispatch;

use crate::{
    data::{prelude::*, spread_id::SpreadId},
    state::prelude::*,
    web::js::get_referrer,
};

use super::data_state::DataState;

// cSpell:ignore xaat

/// This token can only be used to ingest data into our bucket
const API_TOKEN: &str = "xaat-ba30896b-604b-4837-8924-ec8097e55eee";

#[derive(Debug, Clone, Serialize)]
pub struct EventLog {
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "is_false")]
    pub resent: bool,
    pub event: LoggableEvent,
    #[serde(skip_serializing_if = "is_info_or_lower")]
    pub severity: Severity,
}

fn is_false(b: &bool) -> bool {
    !b
}

fn is_info_or_lower(severity: &Severity) -> bool {
    severity != &Severity::Warn && severity != &Severity::Error
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

impl EventLog {
    pub fn new_resent(user_id: Uuid, event: LoggableEvent) -> Self {
        let severity = event.get_severity();
        Self {
            user_id,
            resent: true,
            event,
            severity,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, EnumDiscriminants)]
#[serde(tag = "type")]
pub enum LoggableEvent {
    NewUser {
        ref_param: Option<String>,
        referrer: Option<String>,
        gclid: Option<String>,
        user_agent: String,
        language: String,
    },
    NewSpread {
        question_data: QuestionData,
        spread_id: String,
    },
    ClickShare {
        src_data: SrcData,
    },
    ShareOn {
        platform: SharePlatform,
    },

    Social {
        platform: SocialPlatform,
    },

    Achievement {
        achievement: Achievement,
    },
    ReceivedShare {
        ref_param: Option<String>,
        referrer: Option<String>,
        spread_id: Option<String>,
        img_id: Option<String>,
    },

    Cheat {
        cards: String,
    },
    Custom {
        cards: String,
    },

    Warn {
        message: String,
    },
    Error {
        message: String,
    },
    ViewDailyReading{}
}

impl LoggableEvent {

    pub fn try_log_error(message: String){
        log::error!("{}", message);
        let event = LoggableEvent::Error { message };

        Self::try_log(event)

    }

    pub fn try_log(data: impl Into<Self>) {
        let user = Dispatch::<UserState>::new().get();
        let event = data.into();
        let severity = event.get_severity();
        if let Some(user_id) = user.user_id {
            let message = EventLog {
                event,
                user_id,
                resent: false,
                severity,
            };
            message.send_log();
        } else {
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(event));
            log::error!("User Id not set");
        }
    }

    pub fn get_severity(&self) -> Severity {
        match self {
            LoggableEvent::Warn { .. } => Severity::Warn,
            LoggableEvent::Error { .. } => Severity::Error,
            _ => Severity::Info,
        }
    }

    pub fn new_spread(data: &DataState) -> Self {
        let question_data = data.question_data;
        let spread_id = SpreadId::new(&question_data, &data.perm).encode();

        Self::NewSpread {
            question_data,
            spread_id,
        }
    }

    pub fn new_share(
        ref_param: Option<String>,
        spread_id: Option<String>,
        img_id: Option<String>,
    ) -> Self {
        let referrer = get_referrer();
        let referrer = referrer.is_empty().not().then_some(referrer);
        Self::ReceivedShare {
            referrer,
            ref_param,
            spread_id,
            img_id,
        }
    }
}

impl From<Achievement> for LoggableEvent {
    fn from(achievement: Achievement) -> Self {
        Self::Achievement { achievement }
    }
}

impl From<SocialPlatform> for LoggableEvent {
    fn from(platform: SocialPlatform) -> Self {
        Self::Social { platform }
    }
}

impl From<SharePlatform> for LoggableEvent {
    fn from(platform: SharePlatform) -> Self {
        Self::ShareOn { platform }
    }
}

impl From<SrcData> for LoggableEvent {
    fn from(src_data: SrcData) -> Self {
        Self::ClickShare { src_data }
    }
}

impl EventLog {
    pub fn send_log(self) {
        wasm_bindgen_futures::spawn_local(Self::log(self));
    }

    async fn try_log<T: Serialize>(data: &T) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.axiom.co/v1/datasets/intarot_usage/ingest")
            // .header("Authorization", format!("Bearer {API_TOKEN}"))
            .bearer_auth(API_TOKEN)
            .header("Content-Type", "application/json")
            .json(&[data])
            .send()
            .await?;

        res.error_for_status().map(|_| ())
    }

    async fn log(data: Self) {
        let r = Self::try_log(&data).await;
        if let Err(err) = r {
            log::error!("Failed to log: {}", err);
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(data.event));
        } else {
            let discriminant: LoggableEvent = data.event.into();
            log::debug!("Log {discriminant:?} sent successfully",);
        }
    }
}
