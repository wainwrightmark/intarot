use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties, classes};
use yew_router::prelude::use_navigator;

use crate::web::app::Route;

#[derive(Properties, Debug, PartialEq, Clone, Copy)]
pub struct LogoProps {
    pub clickable: bool,
    pub invertible: bool
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

    let mut classes = classes!("logo");
    if props.invertible{
        classes.push("invertible");
    }

    html!(
    <img   {style} class={classes} alt="intarot logo" {onclick} src="/logo/full logo small.svg"/>
    )
}
