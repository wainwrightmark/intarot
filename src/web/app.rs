use crate::state::failed_logs_state::FailedLogsState;
use crate::state::prelude::*;
use crate::web::landing_view::LandingView;
use crate::web::particles::*;

use yew::prelude::*;

use yew_hooks::use_search_param;
use yew_router::prelude::*;
use yewdux::prelude::Dispatch;

use super::custom_view::CustomView;
use super::prelude::{CheatView, ShareCardView};
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

    #[at("/custom/:cards")]
    Custom { cards: String },

    #[at("/cheat/:cards")]
    Cheat { cards: String },
}

#[function_component(App)]
pub fn app() -> Html {
    let ref_param = use_search_param("ref".to_string()).unwrap_or_default();

    Dispatch::<UserState>::new().apply(CreateUserIfNewMessage { ref_param });
    Dispatch::<FailedLogsState>::new().apply(ResentFailedLogsMessage);

    html! {

        <>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>

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
            <>
            <ParticlesCanvas />
        <SpreadView />
        </>

         },
        Route::Advanced {} => html! {
            <AdvancedView  />
        },
        Route::Share => {
            html!(
                <ShareCardView />
            )
        }
        Route::Cheat { cards } => {
            html!(<CheatView {cards} />)
        }
        Route::Custom { cards } => {
            html!(<CustomView {cards} />)
        }
    }
}
