use yew::prelude::*;
use yew_hooks::use_interval;
use yew_router::prelude::use_navigator;
use yewdux::prelude::{use_store_value, Dispatch};

use super::app::Route;
use crate::{
    state::{
        logging::{Loggable, RequestLog},
        prelude::*,
        prompts_state::PromptsState,
    },
    web::logo::Logo,
};

#[derive(Properties, PartialEq)]
pub struct QuestionProps {}

#[function_component(QuestionView)]
pub fn question_view(_props: &QuestionProps) -> Html {
    let card_page_state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();
    let is_clickable_state = use_state(|| false);

    {
        let is_clickable_state = is_clickable_state.clone();
        let millis = if *is_clickable_state { 0 } else { 5250 };
        use_interval(move || is_clickable_state.set(true), millis);
    };

    let on_begin_click = {
        let navigator = navigator;
        let is_clickable_state = is_clickable_state.clone();
        Callback::from(move |_e: MouseEvent| {
            if *is_clickable_state {
                Dispatch::<DataState>::new().apply(ResetMessage {});

                let data = Dispatch::<DataState>::new().get();
                let user = Dispatch::<UserState>::new().get();
                if let Some(user_id) = user.user_id {
                    let log = RequestLog::new(data.as_ref(), user_id);
                    log.send_log();
                } else {
                    log::error!("User Id not set");
                }

                navigator.replace(&Route::Spread {});
            }
        })
    };

    let background_click = {
        let is_clickable_state = is_clickable_state.clone();
        Callback::from(move |_e: MouseEvent| {
            is_clickable_state.set(true);
        })
    };

    let prompts_state = use_store_value::<PromptsState>();
    let (prompt0, prompt1, prompt2) = prompts_state.get_three_prompts(
        &card_page_state.question_data.guide,
        &card_page_state.question_data.spread_type,
    );
    let prompt0 = format!("Why not ask about {prompt0}?");
    let prompt1 = format!("{prompt1}?");
    let prompt2 = format!("{prompt2}?");

    let button_style = if *is_clickable_state {
        "margin: auto; animation-delay: 5.25s;"
    } else {
        "margin: auto; animation-delay: 5.25s; pointer-events:none;"
    };

    html! {

        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" style="height: 100vh; white-space: nowrap;" onclick={background_click}  >

            <div class="col">
            <div class="row" style="margin-bottom: 0;">
            <Logo clickable={*is_clickable_state}/>
            </div>

            <div class="row">
            <p style="margin: auto; animation-delay: 0s; pointer-events:none;" class={if *is_clickable_state{classes!("")} else {classes!{"fade-in"}}}>{"Take a moment to clear your mind"}</p>
            </div>
            <div class="row">
            <p style="margin: auto; animation-delay: 1.5s; pointer-events:none;" class={if *is_clickable_state {classes!("")} else {classes!{"fade-in"}}}>{"Think of your question"}</p>
            </div>

            <div class="row">
            <p style="margin: auto; animation-delay: 3s; pointer-events:none;" class={if *is_clickable_state {classes!("capitalize_first_letter")} else {classes!{"fade-in", "capitalize_first_letter"}}}>{prompt0 }</p>
            </div>
            <div class="row">
            <p style="margin: auto; animation-delay: 3.75s; pointer-events:none;" class={if *is_clickable_state {classes!("capitalize_first_letter")} else {classes!{"fade-in", "capitalize_first_letter"}}}>{prompt1}</p>
            </div>

            <div class="row">
            <p style="margin: auto; animation-delay: 4.5s; pointer-events:none;" class={if *is_clickable_state {classes!("capitalize_first_letter")} else {classes!{"fade-in", "capitalize_first_letter"}}}>{prompt2}</p>
            </div>
            <div class="row align-middle">
            <button onclick={on_begin_click} style={button_style} class={if *is_clickable_state {classes!("nice-button")} else {classes!{"fade-in", "nice-button"}}}>{"Begin your reading"}</button>
            </div>
            </div>

            </div>
        </div>
        </>
    }
}
