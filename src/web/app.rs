use crate::data::prelude::{Soothsayer, StarSignOption};
use crate::web::opening_view::OpeningView;
use crate::web::soothsayer_view::SoothsayerView;
use yew::prelude::*;
use yew_router::prelude::*;

use super::cards_view::CardsControl;
use crate::web::question_view::QuestionView;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Opening,
    #[at("/:sign")]
    Soothsayer { sign: StarSignOption },

    #[at("/:sign/:soothsayer")]
    Question {
        sign: StarSignOption,
        soothsayer: Soothsayer,
    },

    #[at("/:sign/:soothsayer/:seed")]
    Card {
        sign: StarSignOption,
        soothsayer: Soothsayer,
        seed: u32,
    },
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </HashRouter>

    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Opening => html! {

        <OpeningView />

         },

         Route::Question { sign, soothsayer }=> html!{
            <QuestionView sign={sign.0} soothsayer={soothsayer} />
         },

        Route::Soothsayer { sign } => html! {

            <SoothsayerView sign={sign.0} />

        },
        Route::Card {
            sign,
            soothsayer,
            seed,
        } => html! {

        <CardsControl sign={sign.0} soothsayer={soothsayer} seed={seed}/>

         },
    }
}
