use crate::{state::preferences_state::*, web::prelude::*};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(PreferencesView)]
pub fn preferences_view() -> Html {
    html! {
        <div class="site">
            <div class="container" style="overflow-x: hidden" >
            <div class="contained col min-width-contained">

                <Logo clickable={true} invertible={true}/>

                <DarkModeButton/>
                <br/>
                    <MotionStateButton/>
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

    let onclick = dispatch.reduce_callback(|z| {
        (match *z {
            DarkModeState::Auto => DarkModeState::Light,
            DarkModeState::Light => DarkModeState::Dark,
            DarkModeState::Dark => DarkModeState::Auto,
        })
        .into()
    });

    let text = match *state {
        DarkModeState::Auto => "Dynamic Theme",
        DarkModeState::Light => "Light Theme",
        DarkModeState::Dark => "Dark Theme",
    };

    html!(
        <button style="margin: auto; display: block;" class="nice-button advanced-view-button" {onclick}>
            {text}
        < /button>

    )
}

#[function_component(MotionStateButton)]
fn motion_state_button() -> Html {
    use MotionState::*;

    let (state, dispatch) = use_store::<MotionState>();

    let onclick = dispatch.reduce_callback(|z| {
        (match *z {
            FullMotion => ReducedMotion,
            ReducedMotion => FullMotion,
        })
        .into()
    });

    let text = match *state {
        FullMotion => "Full Motion",
        ReducedMotion => "Reduced Motion",
    };

    html!(
        <button style="margin: auto; display: block;" class="nice-button advanced-view-button" {onclick}>
            {text}
        < /button>

    )
}
