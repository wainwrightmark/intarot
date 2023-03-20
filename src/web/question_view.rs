use capacitor_bindings::haptics::*;
use yew::prelude::*;
use yew_hooks::{use_effect_once, use_interval};
use yew_router::prelude::use_navigator;
use yewdux::prelude::{use_store_value, Dispatch};

use crate::{
    state::{prelude::*, prompts_state::PromptsState},
    web::{capacitor, logo::Logo, prelude::*},
};

#[derive(Properties, PartialEq)]
pub struct QuestionProps {}

#[function_component(QuestionView)]
pub fn question_view(_props: &QuestionProps) -> Html {
    use_effect_once(|| scroll_to_top);
    let data_state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();
    let is_clickable_state = use_state(|| false);

    {
        let is_clickable_state = is_clickable_state.clone();
        let millis = if *is_clickable_state { 0 } else { 5250 };
        use_interval(
            move || {
                is_clickable_state.set(true);
                capacitor::do_or_report_error(|| Haptics::vibrate(300.));
            },
            millis,
        );
    };

    let on_begin_click = {
        let navigator = navigator;
        let is_clickable_state = is_clickable_state.clone();
        Callback::from(move |_e: MouseEvent| {
            if *is_clickable_state {
                Dispatch::<DataState>::new().apply(ResetMessage {});

                let data = Dispatch::<DataState>::new().get();

                LoggableEvent::try_log_new_spread(data.as_ref());

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

    let guide = data_state.question_data.guide;
    let spread_type = data_state.question_data.spread_type;

    let prompts_state = use_store_value::<PromptsState>();
    let (prompt0, prompt1, prompt2) = prompts_state.get_three_prompts(&guide, &spread_type);
    let prompt0 = format!("Why not ask about {prompt0}?");
    let prompt1 = format!("{prompt1}?");
    let prompt2 = format!("{prompt2}?");

    let mut button_classes = classes!("nice-button", "begin-button");

    let mut button_style = format!(
        "--guide-primary: {}; --guide-secondary: {}",
        guide.primary_color(),
        guide.secondary_color()
    );

    if !(*is_clickable_state) {
        button_classes.push("fade-in");
        button_style.push_str(" pointer-events:none;");
    }

    html! {

        <>
        <div class="site">
            <div class="container" style="height: 100vh; white-space: nowrap;" onclick={background_click}  >
                <div class = "col contained min-width-contained">
            <div class="row" style="margin-bottom: 0;">
            <Logo clickable={*is_clickable_state} invertible={true}/>
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
            <div class="row" style="align-self: center;">
            <button onclick={on_begin_click} style={button_style} class={button_classes}>{"Begin your reading"}</button>
            </div>
            </div>
                </div>
        </div>
        </>
    }
}
