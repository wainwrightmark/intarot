use crate::{
    state::{prelude::{ShufflePromptsMessage, ChangeSpreadTypeMessage, DataState}, prompts_state::PromptsState},
    web::prelude::*,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::Dispatch;

#[function_component(LandingView)]
pub fn landing_view() -> Html {
    let navigator = use_navigator().unwrap();

    let paragraph1 = include_str!(r#"../text/opening_p1.txt"#);
    let paragraph2 = include_str!(r#"../text/opening_p2.txt"#);
    let paragraph3 = include_str!(r#"../text/opening_p3.txt"#);

    let on_begin_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            Dispatch::<PromptsState>::new().apply(ShufflePromptsMessage);

            Dispatch::<DataState>::new().apply(ChangeSpreadTypeMessage(crate::data::prelude::SpreadType::One ));
            navigator.push(&Route::Question {});
        })
    };

    let on_advanced_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Advanced {});
        })
    };

    html! {
        <div class="site">
            <div class="container" >
            <div class="sm-4 col" style="margin: auto; text-align: justify;">

            <Logo clickable={true}/>
            <p class="landing-paragraph">
            {paragraph1}
            </p>
            <p class="landing-paragraph">
            {paragraph2}
            </p>
            <p class="landing-paragraph">
            {paragraph3}
            </p>
                </div>
                <button onclick={on_begin_click} style="margin: auto; display: block;" class="nice-button">{"Begin"}</button>
                <br/>
                <button onclick={on_advanced_click} style="margin: auto; display: block;" class="nice-button">{"Advanced"}</button>
                <br/>
                <br/>
            </div>
        </div>
    }
}