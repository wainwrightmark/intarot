use yew::prelude::*;
use yewdux::prelude::*;

use crate::state::prelude::*;
use crate::web::cards_view::CardsControl;
use crate::web::opening_view::OpeningView;
use crate::web::soothsayer_view::SoothsayerView;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="site">
            <div class="container" >
            <Content/>
            </div>
        </div>
    }
}

#[function_component(Content)]
pub fn content() -> Html {
    let (image_state, _) = use_store::<PageState>();

    match image_state.as_ref() {
        PageState::OpeningPage(_) => html!(<OpeningView />),
        PageState::SoothsayerPage(_) => html!(<SoothsayerView />),
        PageState::CardPage(_) => html!(<CardsControl />),
    }
}
