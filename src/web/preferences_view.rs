use crate::{
    state::preferences_state::{
        CardShakeState, CardShakeToggleMessage, DarkModeState, DarkModeToggleMessage,
    },
    web::prelude::*,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(PreferencesView)]
pub fn preferences_view() -> Html {
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
                <BackButton/>
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
