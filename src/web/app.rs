use crate::state::failed_logs_state::FailedLogsState;
use crate::state::prelude::*;
use crate::web::landing_view::LandingView;
use crate::web::particles::*;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use yew_hooks::{use_effect_once, use_search_param};
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
    NoRoute,

    #[at("/landing")]
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

async fn setup(ref_param: Option<String>, gclid_param: Option<String>) {
    Dispatch::<UserState>::new()
        .apply_future(UpdateParamsIfNewMessage {
            ref_param,
            gclid_param,
        })
        .await;
    Dispatch::<FailedLogsState>::new()
        .apply_future(ResentFailedLogsMessage)
        .await;

    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error_async(|| async { StatusBar::set_style(Style::Light).await })
            .await;
        crate::web::capacitor::do_or_report_error_async(|| async {
            StatusBar::set_background_color("#FFFFFF").await
        })
        .await;
    }

    crate::setup_notifications_async().await;
}

#[function_component(App)]
pub fn app() -> Html {
    let ref_param = use_search_param("ref".to_string());
    let gclid_param = use_search_param("gclid".to_string());

    use_effect_once(|| {
        spawn_local(setup(ref_param, gclid_param));
        || ()
    });

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
        Route::NoRoute {} => html! {
            <AdvancedView  />
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
