use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::{prelude::Card, question_data::QuestionData};

use super::data_state::DataState;

/// This token can only be used to ingest data into our bucket
const API_TOKEN: &'static str = "xaat-ba30896b-604b-4837-8924-ec8097e55eee";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub user_id: Uuid,
    pub question_data: QuestionData,
    pub variant_seed: u32,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserLog {
    pub user_id: Uuid,
    pub referrer: Option<String>,
}

impl RequestLog {
    pub fn new(data: &DataState, user_id: Uuid) -> Self {
        let cards_to_take = data.question_data.spread_type.total_cards().min(7);

        Self {
            question_data: data.question_data,
            variant_seed: data.variant_seed,
            cards: data.cards.iter().take(cards_to_take).cloned().collect_vec(),
            user_id,
        }
    }
}

impl Loggable for RequestLog {}
impl Loggable for NewUserLog {}

pub trait Loggable: Sized + Serialize + 'static {
    fn send_log(self) -> () {
        wasm_bindgen_futures::spawn_local(log(self));
    }
}

async fn try_log<T: Serialize>(data: T) -> Result<(), reqwest::Error> {
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

async fn log<T: Serialize>(data: T) -> () {
    let r = try_log(data).await;
    if let Err(err) = r {
        log::error!("Logging Error {}", err);
    } else {
        log::debug!("Log sent successfully");
    }
}
