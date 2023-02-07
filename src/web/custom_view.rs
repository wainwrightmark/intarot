use std::str::FromStr;

use crate::data::image_data::{ImageData, ImageType};
use crate::data::prelude::ImageMeta;
use crate::state::prelude::*;
use crate::web::logo::Logo;
use itertools::Itertools;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CustomViewProps {
    pub cards: String,
}

#[function_component(CustomView)]
pub fn custom_view(props: &CustomViewProps) -> Html {
    let navigator = use_navigator();

    let custom_spread = get_custom_spread(props.cards.as_str());

    Dispatch::<DataState>::new().apply(SetCustomSpreadMessage {
        custom: custom_spread.into(),
    });

    Dispatch::<DataState>::new().apply(BackToTopMessage);

    navigator.unwrap().push(&crate::web::app::Route::Spread);
    let event = LoggableEvent::Custom {
        cards: props.cards.clone(),
    };
    LoggableEvent::try_log(event);

    html!(<> <Logo clickable={true}/> </> )
}

fn get_custom_spread(str: &str) -> CustomSpread {
    let cards = str
        .split_terminator(',')
        .map(|id| match ImageMeta::from_str(id) {
            Ok(im) => im.image_data,
            Err(_) => ImageData {
                id: id.to_string().into(),
                image_type: ImageType::Custom,
            },
        })
        .collect_vec();
    CustomSpread { cards }
}
