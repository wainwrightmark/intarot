use crate::{data::achievement::Achievement, state::prelude::*, web::prelude::*};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;
use yewdux::prelude::Dispatch;

#[function_component(LandingView)]
pub fn landing_view() -> Html {
    let navigator = use_navigator().unwrap();
    use_effect_once(|| scroll_to_top);

    let paragraph1 = include_str!(r#"../text/opening_p1.txt"#);
    let paragraph2 = include_str!(r#"../text/opening_p2.txt"#);
    // let paragraph3 = include_str!(r#"../text/opening_p3.txt"#);

    let on_begin_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::LandingClickBegin));
            Dispatch::<PromptsState>::new().apply(ShufflePromptsMessage);

            Dispatch::<DataState>::new().apply(ChangeSpreadTypeMessage(
                crate::data::prelude::SpreadType::Three,
            ));
            navigator.push(&Route::Question {});
        })
    };

    let on_advanced_click = {
        let navigator = navigator;

        Callback::from(move |_: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::LandingClickAdvanced));
            navigator.push(&Route::Advanced {});
        })
    };

    html! {
        <div class="site">
            <div class="container" >
            <div class="contained col min-width-contained">

            <Logo clickable={true}/>

            <button onclick={on_begin_click} style="margin: auto; display: block;" class="nice-button advanced-view-button">{"Quick Reading"}</button>
                <br/>
                <p class="landing-paragraph">
                    {paragraph1}
                </p>
                <br/>
            <button onclick={on_advanced_click} style="margin: auto; display: block;" class="nice-button advanced-view-button">{"Advanced Reading"}</button>
                <br/>
                <p class="landing-paragraph">
                    {paragraph2}
                </p>
                <br/>
                </div>


                <footer class={"advanced-view-item"}  >
                <SocialIcons />
                <br/>
                </footer>
            </div>
        </div>
    }
}
