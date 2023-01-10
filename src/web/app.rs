use crate::data::prelude::{Guide, StarSignOption};
use crate::web::guide_view::GuideView;
use crate::web::opening_view::OpeningView;
use yew::prelude::*;
use yew_router::prelude::*;

use super::spread_view::SpreadView;
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
    Guide { sign: StarSignOption, guide: Guide },

    #[at("/:sign/:guide")]
    Question { sign: StarSignOption, guide: Guide },

    #[at("/:sign/:guide/:seed")]
    Card {
        sign: StarSignOption,
        guide: Guide,
        seed: u32,
    },

    #[at("/Restart/:sign/:guide/")]
    Restart { sign: StarSignOption, guide: Guide },
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

        Route::Question { sign, guide } => html! {
           <QuestionView sign={sign.0} guide={guide} />
        },

        Route::Guide { sign, guide } => html! {

            <GuideView sign={sign.0} go_to_question={false} guide={guide} />

        },
        Route::Card { sign, guide, seed } => html! {

        <SpreadView sign={sign.0} guide={guide} seed={seed}/>

         },
        Route::Restart { sign, guide } => html! {
            <RestartView sign={sign.0} guide={guide} />
        },
        Route::Choose => html! {

            <GuideView sign={None} go_to_question={true} />

        },
    }
}
