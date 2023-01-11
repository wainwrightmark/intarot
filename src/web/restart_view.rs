use std::str::FromStr;

use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use super::app::Route;
use crate::{data::prelude::*, state::prelude::*};

#[derive(Properties, PartialEq)]
pub struct RestartProps {}

#[function_component(RestartView)]
pub fn restart_view(_props: &RestartProps) -> Html {
    let navigator = use_navigator().unwrap();
    let state = use_store_value::<DataState>();
    let user_data = state.user_data.clone();
    let on_sign_change = {
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let s = input.value();
            let star_sign = StarSign::from_str(s.as_str()).ok();
            let mut user_data = state.user_data;
            user_data.star_sign = star_sign;
            Dispatch::<DataState>::new().apply(MaybeChangeDataMessage(user_data))
        })
    };

    let on_spread_type_change = {
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let s = input.value();
            let spread_type = SpreadType::from_str(s.as_str()).unwrap();
            let mut user_data = user_data;
            user_data.spread_type = spread_type;
            Dispatch::<DataState>::new().apply(MaybeChangeDataMessage(user_data))
        })
    };

    let sign_options = StarSign::iter()
    .map(|star_sign| {
        let selected = Some(star_sign) == user_data.star_sign;
        html!(  <option value={star_sign.repr()} {selected} disabled={false}> {star_sign.name()}  </option>
        )
    })
    .collect_vec();

    let spread_type_options = SpreadType::iter()
    .map(|spread_type|{

        let selected = spread_type == user_data.spread_type;
        html!(  <option value={spread_type.repr()} {selected} disabled={false}> {spread_type.name()}  </option>
        )
    }

    ).collect_vec();

    let on_begin_click = {
        let navigator = navigator.clone();
        Callback::from(move |_e: MouseEvent| {
            navigator.push(&Route::Question {});
        })
    };

    let on_image_click = {
        let navigator = navigator.clone();
        Callback::from(move |_e: MouseEvent| {
            navigator.push(&Route::Guide {});
        })
    };

    html! {

            <>
            <div class="site" style="overflow: hidden ">
                <div class="container" style="height: 100vh;"  >
                <h3 style="color: gold; text-align: center;">
            {"intarot"}
            </h3>

            <div class={"restart-image"}  >
            <img onclick={on_image_click} class="guide-image"
            src={format!("https://drive.google.com/uc?export=view&id={}", user_data.guide.image_id()) }
                 alt={user_data.guide.name()} />
            </div>
            <br/>

            <div>
            <select onchange={on_sign_change} class="nice-button" style="margin:auto; width:200px;">
            <option selected={user_data.star_sign.is_none()} disabled={false}> {"Star Sign"}  </option>
            {sign_options}
            </select>
        <br/>
            </div>
            <div>
            <select onchange={on_spread_type_change} class="nice-button" style="margin:auto; width:200px;">
            {spread_type_options}
            </select>

            </div>
            <br/>
            <button onclick={on_begin_click} style="margin: auto; display: block; width:200px;" class="nice-button">{"Begin"}</button>
    </div>
                </div>
            </>
        }
}
