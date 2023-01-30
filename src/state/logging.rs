use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yewdux::prelude::Dispatch;

use crate::{
    data::prelude::*,
    state::{failed_logs_state::FailedLogsState, messages::*},
};

use super::data_state::DataState;

/// This token can only be used to ingest data into our bucket
const API_TOKEN: &str = "xaat-ba30896b-604b-4837-8924-ec8097e55eee";

#[derive(Debug, Clone, Serialize)]
pub struct EventLog {
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "is_false")]
    pub resent: bool,
    pub event: LoggableEvent,
}

fn is_false(b: &bool) -> bool {
    !b
}

impl EventLog {
    pub fn new(user_id: Uuid, event: LoggableEvent) -> Self {
        Self {
            user_id,
            resent: false,
            event,
        }
    }
    pub fn new_resent(user_id: Uuid, event: LoggableEvent) -> Self {
        Self {
            user_id,
            resent: true,
            event,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoggableEvent {
    NewUser {
        referrer: Option<String>,
    },
    NewSpread {
        question_data: QuestionData,
        variant_seed: u32,
        cards: Vec<Card>,
    },
    Share {
        src_data: SrcData2,
    },
    Achievement {
        achievement: Achievement,
    },
}

impl LoggableEvent {
    pub fn new_spread(data: &DataState) -> Self {
        let cards_to_take = data.question_data.spread_type.total_cards().min(7);
        let question_data = data.question_data;
        let variant_seed = data.variant_seed;
        let cards = data.cards.iter().take(cards_to_take).cloned().collect_vec();

        Self::NewSpread {
            question_data,
            variant_seed,
            cards,
        }
    }
}

impl From<Achievement> for LoggableEvent {
    fn from(achievement: Achievement) -> Self {
        Self::Achievement { achievement }
    }
}

impl From<SrcData> for LoggableEvent {
    fn from(src_data: SrcData) -> Self {
        Self::Share {
            src_data: src_data.into(),
        }
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
            log::error!("Logging Error {}", err);
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(data.event));
        } else {
            log::debug!("Log sent successfully");
        }
    }
}

// // impl Loggable for EventLog {}

// pub trait Loggable: Sized + Serialize + 'static {

// }
