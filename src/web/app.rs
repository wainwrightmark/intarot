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

    #[at("/guide")]
    Guide {},

    #[at("/question")]
    Question {},

    #[at("/spread")]
    Spread {},

    #[at("/restart")]
    Restart {},
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
        Route::Opening => {
            html! {
               html!{
                   <OpeningView />
               }
            }
        }

        Route::Question {} => html! {
           <QuestionView  />
        },

        Route::Guide {} => html! {

            <GuideView  go_to_question={false}  />

        },
        Route::Spread {} => html! {

        <SpreadView />

         },
        Route::Restart {} => html! {
            <RestartView  />
        },
        Route::Choose => html! {

            <GuideView go_to_question={true} />

        },
    }
}
