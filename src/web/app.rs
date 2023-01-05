use crate::data::prelude::{Ordering, Soothsayer, StarSign};
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
    Soothsayer { sign: StarSign },

    #[at("/:sign/:soothsayer")]
    Question {
        sign: StarSign,
        soothsayer: Soothsayer,
    },

    #[at("/:sign/:soothsayer/:ordering")]
    Card {
        sign: StarSign,
        soothsayer: Soothsayer,
        ordering: Ordering,
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
            <QuestionView sign={sign} soothsayer={soothsayer} />
         },

        Route::Soothsayer { sign } => html! {

            <SoothsayerView sign={sign} />

        },
        Route::Card {
            sign,
            soothsayer,
            ordering,
        } => html! {

        <CardsControl sign={sign} soothsayer={soothsayer} ordering={ordering}/>

         },
    }
}
