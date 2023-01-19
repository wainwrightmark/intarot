use crate::web::landing_view::LandingView;
use yew::prelude::*;
use yew_router::prelude::*;

use super::prelude::ShareCardView;
use super::spread_view::SpreadView;
use crate::web::question_view::QuestionView;
use crate::web::advanced_view::AdvancedView;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Landing,
    #[at("/question")]
    Question,

    #[at("/spread")]
    Spread,

    #[at("/advanced")]
    Advanced,

    #[at("/share")]
    Share,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>

    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Landing => {
            html! {
               html!{
                   <LandingView />
               }
            }
        }

        Route::Question {} => html! {
           <QuestionView  />
        },
        Route::Spread {} => html! {

        <SpreadView />

         },
        Route::Advanced {} => html! {
            <AdvancedView  />
        },
        Route::Share => {
            html!(
                <ShareCardView />
            )
        }
    }
}
