use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::{use_store_value, Dispatch};

use super::app::Route;
use crate::state::{prelude::*, prompts_state::PromptsState};

#[derive(Properties, PartialEq)]
pub struct QuestionProps {}

#[function_component(QuestionView)]
pub fn question_view(_props: &QuestionProps) -> Html {
    let card_page_state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();
    let skipped_state = use_state(|| false);

    let on_begin_click = {
        let navigator = navigator;
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<DataState>::new().apply(ResetMessage {});
            navigator.replace(&Route::Spread {});
        })
    };



    let background_click = {
        let skipped_state = skipped_state.clone();
        Callback::from(move |_e: MouseEvent| {
            skipped_state.set(true);
        })
    };

    let prompts_state = use_store_value::<PromptsState>();
    let (prompt0, prompt1, prompt2) = prompts_state.get_three_prompts(&card_page_state.question_data.guide);
    let prompt0 = format!("Why not ask about {prompt0}?") ;
    let prompt1 = format!("{prompt1}?");
    let prompt2 = format!("{prompt2}?");


    html! {

            <>
            <div class="site" style="overflow: hidden ">
                <div class="container" style="height: 100vh;" onclick={background_click}  >
                <h3 style="color: gold; text-align: center; pointer-events:none;">
            {"intarot"}
            </h3>
                <div class="col">
                <div class="row">
                <p style="margin: auto; animation-delay: 0s; pointer-events:none;" class={if *skipped_state{classes!("")} else {classes!{"fade-in"}}}>{"Take a moment to clear your mind"}</p>
                </div>
                <div class="row">
                <p style="margin: auto; animation-delay: 1.5s; pointer-events:none;" class={if *skipped_state {classes!("")} else {classes!{"fade-in"}}}>{"Think of your question"}</p>
                </div>

                <div class="row">
                <p style="margin: auto; animation-delay: 3s; pointer-events:none;" class={if *skipped_state {classes!("capitalize_first_letter")} else {classes!{"fade-in", "capitalize_first_letter"}}}>{prompt0 }</p>
                </div>
                <div class="row">
                <p style="margin: auto; animation-delay: 3.75s; pointer-events:none;" class={if *skipped_state {classes!("capitalize_first_letter")} else {classes!{"fade-in", "capitalize_first_letter"}}}>{prompt1}</p>
                </div>

                <div class="row">
                <p style="margin: auto; animation-delay: 4.5s; pointer-events:none;" class={if *skipped_state {classes!("capitalize_first_letter")} else {classes!{"fade-in", "capitalize_first_letter"}}}>{prompt2}</p>
                </div>
                <div class="row align-middle">
                <button onclick={on_begin_click} style="margin: auto; animation-delay: 5.25s;" class={if *skipped_state {classes!("nice-button")} else {classes!{"fade-in", "nice-button"}}}>{"Begin your reading"}</button>
                </div>
                </div>
                <div class="row">
    </div>
                </div>
            </div>
            </>
        }
}
