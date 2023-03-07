use std::{collections::HashMap, rc::Rc, str::FromStr};

use crate::data::prelude::*;
use itertools::Itertools;
use yewdux::prelude::*;

use super::logging::LoggableEvent;

#[derive(PartialEq, Eq, Store, Default, Clone)]
pub struct ImageMetaState {
    pub metas: HashMap<MetaKey, Vec<ImageMeta>>,
}

pub struct SetUpImageMetaStateMessage;

#[async_reducer]
impl AsyncReducer<ImageMetaState> for SetUpImageMetaStateMessage {
    async fn apply(self, mut image_meta_state: Rc<ImageMetaState>) -> Rc<ImageMetaState> {
        let state = Rc::make_mut(&mut image_meta_state);
        let url = "https://intarot-images.s3.eu-west-2.amazonaws.com/Data/image_names.tsv";

        let data = reqwest::Client::builder()
            .build()
            .unwrap()
            .get(url)
            .send()
            .await;

        match data {
            Ok(response) => match response.text().await {
                Ok(text) => {
                    let lines = text.lines();

                    let metas = lines
                        .map(move |str| ImageMeta::from_str(str).unwrap())
                        .into_group_map_by(|image_meta| MetaKey::from(image_meta.clone()));
                    state.metas = metas;
                }
                Err(err) => {
                    LoggableEvent::try_log_error_async(err).await;
                }
            },
            Err(err) => {
                LoggableEvent::try_log_error_async(err).await;
            }
        }

        image_meta_state
    }
}
