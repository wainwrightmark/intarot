use std::str::FromStr;

use crate::data::prelude::ImageMeta;
use crate::web::guide_view::GuideView;
use crate::web::opening_view::OpeningView;
use yew::prelude::*;
use yew_router::prelude::*;

use super::prelude::ShareCardView;
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

    #[at("/share/:encoded_image_name")]
    Share{encoded_image_name: String}
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
        Route::Share { encoded_image_name } => {
            let image_meta = base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE, encoded_image_name)
            .ok()
            .and_then(|x| String::from_utf8(x).ok())
            .and_then(|x| ImageMeta::from_str(x.as_str()).ok())
            ;

            if let Some(image_meta) = image_meta{
                html!(
                    <ShareCardView {image_meta} />
                )
            }
            else{
                html!(
                    <OpeningView />
                )
            }

        },
    }
}
