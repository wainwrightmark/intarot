use std::str::FromStr;

use itertools::Itertools;
use yew::prelude::*;

use yew_hooks::use_search_param;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::description_layout::DescriptionLayout;
use crate::data::prelude::{ImageMeta, SrcData};
use crate::data::spread_id::SpreadId;
use crate::state::prelude::*;
use crate::web::card_view::*;
use crate::web::logo::Logo;

#[derive(Properties, PartialEq)]
pub struct CustomViewProps {
    pub cards: String,
}

#[function_component(CustomView)]
pub fn custom_view(props: &CustomViewProps) -> Html {
    let navigator = use_navigator();
    //let descriptions_state = use_store_value::<ImageDescriptionState>();
    //let description_layout: DescriptionLayout = Default::default();

    let custom_spread = get_custom_spread(props.cards.as_str());

    Dispatch::<DataState>::new().apply(SetCustomSpreadMessage {
        custom: custom_spread.into(),
    });
    navigator.unwrap().push(&crate::web::app::Route::Spread);
    // let event = LoggableEvent::new_share(referrer, spread, id.clone());
    // LoggableEvent::try_log(event);

    // if let Some((qd, perm)) = spread_data {
    //     Dispatch::<DataState>::new().apply(LoadSpreadMessage(qd, perm));
    //     navigator.unwrap().push(&crate::web::app::Route::Spread);
    // } else if id.is_none() {
    //     navigator.unwrap().push(&crate::web::app::Route::Landing);
    // }

    // let image_meta = ImageMeta::from_str(id.unwrap_or_default().as_str()).ok();

    html!(<> <Logo clickable={true}/> </> )
}

fn get_custom_spread(str: &str) -> CustomSpread {
    let cards = str
        .split_terminator(",")
        .map(|x| std::sync::Arc::new(x.to_string()))
        .collect_vec();
    CustomSpread { cards }
}
