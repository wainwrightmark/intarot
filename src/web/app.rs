use crate::data::prelude::{Soothsayer, StarSignOption};
use crate::web::opening_view::OpeningView;
use crate::web::soothsayer_view::SoothsayerView;
use yew::prelude::*;
use yew_router::prelude::*;

use super::cards_view::CardsControl;
use crate::web::question_view::QuestionView;
use crate::web::restart_view::RestartView;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Opening,

    #[at("/choose")]
    Choose,

    #[at("/choose/:sign/:guide")]
    Soothsayer { sign: StarSignOption, guide: Soothsayer },

    #[at("/:sign/:guide")]
    Question {
        sign: StarSignOption,
        guide: Soothsayer,
    },

    #[at("/:sign/:soothsayer/:seed")]
    Card {
        sign: StarSignOption,
        soothsayer: Soothsayer,
        seed: u32,
    },

    #[at("/Restart/:sign/:soothsayer/")]
    Restart{
        sign: StarSignOption,
        soothsayer: Soothsayer,
    }
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

         Route::Question { sign, guide: soothsayer }=> html!{
            <QuestionView sign={sign.0} soothsayer={soothsayer} />
         },

        Route::Soothsayer { sign, guide } => html! {

            <SoothsayerView sign={sign.0} go_to_question={false} guide={guide} />

        },
        Route::Card {
            sign,
            soothsayer,
            seed,
        } => html! {

        <CardsControl sign={sign.0} soothsayer={soothsayer} seed={seed}/>

         },
        Route::Restart { sign, soothsayer } => html!{
            <RestartView sign={sign.0} guide={soothsayer} />
        },
        Route::Choose => html! {

            <SoothsayerView sign={None} go_to_question={true} />

        },
    }
}
