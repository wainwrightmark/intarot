use crate::state::prelude::{UserState, CreateUserIfNewMessage};
use crate::web::landing_view::LandingView;
use yew::prelude::*;
use yew_hooks::use_search_param;
use yew_router::prelude::*;
use yewdux::prelude::Dispatch;

use super::prelude::ShareCardView;
use super::spread_view::SpreadView;
use crate::web::advanced_view::AdvancedView;
use crate::web::question_view::QuestionView;

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
    let referrer = use_search_param("ref".to_string());

    let dispatch = Dispatch::<UserState>::new();
    dispatch.apply(CreateUserIfNewMessage{referrer});
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
