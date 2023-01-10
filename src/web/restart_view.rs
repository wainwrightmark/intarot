use std::str::FromStr;

use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use super::app::Route;
use crate::data::prelude::{Guide, StarSign};

#[derive(Properties, PartialEq)]
pub struct RestartProps {
    pub sign: Option<StarSign>,
    pub guide: Guide,
}

#[function_component(RestartView)]
pub fn restart_view(props: &RestartProps) -> Html {
    let sign_state = use_state(|| props.sign);

    let on_sign_change = {
        let sign_state = sign_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let s = input.value();

            if let Ok(sign) = StarSign::from_str(s.as_str()) {
                sign_state.set(Some(sign));
            } else {
                sign_state.set(None);
            }
        })
    };

    let sign_options = StarSign::iter()
    .map(|value| {
        let selected = Some(value) == props.sign;
        html!(  <option value={value.repr()} {selected} disabled={false}> {value.name()}  </option>
        )
    })
    .collect_vec();

    let onclick = {
        let guide = props.guide;
        let navigator = use_navigator().unwrap();
        let sign_state = sign_state.clone();
        Callback::from(move |_e: MouseEvent| {
            navigator.push(&Route::Question {
                sign: (*sign_state).into(),
                guide,
            });
        })
    };

    let on_image_click = {
        let navigator = use_navigator().unwrap();
        let sign_state = sign_state.clone();
        let guide = props.guide;
        Callback::from(move |_e: MouseEvent| {
            navigator.push(&Route::Guide {
                sign: (*sign_state).into(),
                guide,
            });
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
            src={format!("https://drive.google.com/uc?export=view&id={}", props.guide.image_id()) }
                 alt={props.guide.name()} />
            </div>
            <br/>

            <div>
            <select onchange={on_sign_change} class="nice-button" style="margin:auto; width:200px;">
            <option selected={sign_state.is_none()} disabled={true}> {"Star Sign"}  </option>
            <option selected={sign_state.is_none()} disabled={false}> {"Undecided"}  </option>
            {sign_options}
        </select>
        <br/>
            </div>
            <div>
            <select class="nice-button" style="margin:auto; width:200px;">
            <option  disabled={true} selected={true} >  {"Reading Type"}  </option>
            <option  disabled={false} > {"Single Card"}  </option>
            <option  disabled={false} > {"Three Cards"}  </option>
            <option  disabled={false} > {"Browse"}  </option>
        </select>

            </div>
            <br/>
            <button {onclick} style="margin: auto; display: block; width:200px;" class="nice-button">{"Begin"}</button>
    </div>
                </div>
            </>
        }
}
