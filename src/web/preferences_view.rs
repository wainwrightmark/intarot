use crate::{
    state::preferences_state::{
        CardShakeState, CardShakeToggleMessage, DarkModeState, DarkModeToggleMessage,
    },
    web::prelude::*,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[function_component(PreferencesView)]
pub fn preferences_view() -> Html {
    let navigator = use_navigator().unwrap();

    let on_back_click = {
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Landing {});
        })
    };

    html! {
        <div class="site">
            <div class="container" style="overflow-x: hidden" >
            <div class="contained col min-width-contained">

                <Logo clickable={true} invertible={true}/>

                <div class="checkbox-wrapper">
                    <DarkModeButton/>
                    <CardShakeButton/>
                </div>
                <br/>
                <br/>
                <button onclick={on_back_click} style="margin: auto; display: block;" class="nice-button advanced-view-button">{"Back"}</button>
                <br/>
                <br/>
                <div class={"advanced-view-item"}  >
        <SocialIcons />
        </div>

            </div>


        </div>
    </div>
    }
}

#[function_component(DarkModeButton)]
fn dark_mode_button() -> Html {
    let (state, dispatch) = use_store::<DarkModeState>();

    let onchange = dispatch.apply_callback(|_| DarkModeToggleMessage);

    html!(
        <>
        <input type="checkbox" id="dm" checked={state.is_dark} {onchange}/>
           <label for="dm">
            {"Dark Mode"}
           </label>
           </>
    )
}

#[function_component(CardShakeButton)]
fn card_shake_button() -> Html {
    let (state, dispatch) = use_store::<CardShakeState>();

    let onchange = dispatch.apply_callback(|_| CardShakeToggleMessage);

    html!(
        <>
        <input type="checkbox" id="dm" checked={state.enabled} {onchange}/>
           <label for="dm">
            {"Card Shake"}
           </label>
           </>
    )
}
