use crate::{
    state::{prelude::ShufflePromptsMessage, prompts_state::PromptsState},
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

    let onclick = Callback::from(move |_: MouseEvent| {
        Dispatch::<PromptsState>::new().apply(ShufflePromptsMessage);
        navigator.push(&Route::Question {});
    });

    html! {
        <div class="site">
            <div class="container" >
            <div class="sm-4 col" style="margin: auto; text-align: justify;">

            <Logo clickable={true}/>
            <p>
            {paragraph1}
            </p>
            <p>
            {paragraph2}
            </p>
            <p>
            {paragraph3}
            </p>
                </div>
                <button {onclick} style="margin: auto; display: block;" class="nice-button">{"Begin"}</button>
            </div>
        </div>
    }
}
