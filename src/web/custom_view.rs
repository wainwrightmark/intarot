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
        .split_terminator(",")
        .map(|x| std::sync::Arc::new(x.to_string()))
        .collect_vec();
    CustomSpread { cards }
}
