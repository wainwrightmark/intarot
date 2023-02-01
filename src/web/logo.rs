use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::web::app::Route;

#[derive(Properties, Debug, PartialEq, Clone, Copy)]
pub struct LogoProps {
    pub clickable: bool,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let navigator = use_navigator().unwrap();

    let style = if props.clickable {
        "cursor:pointer;"
    } else {
        "pointer-events: none;"
    };

    let onclick = {
        let props = *props;
        Callback::from(move |_: MouseEvent| {
            if props.clickable {
                navigator.push(&Route::Landing {});
            }
        })
    };

    html!(
    <img  width="200" height="133" {style} class="logo" alt="intarot logo" {onclick} src="logo/full logo small.svg"/>
    )
}
