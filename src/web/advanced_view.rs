use std::str::FromStr;

use itertools::Itertools;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use super::app::Route;
use crate::{
    data::prelude::*,
    state::{prelude::*, prompts_state::PromptsState},
    web::{logo::Logo, prelude::GuideCarousel},
};

#[derive(Properties, PartialEq)]
pub struct AdvancedProps {}

#[function_component(AdvancedView)]
pub fn advanced_view(_props: &AdvancedProps) -> Html {
    let navigator = use_navigator().unwrap();
    let data_state = use_store_value::<DataState>();
    let description_state = use_store_value::<SpreadDescriptionState>();

    let user_data = data_state.question_data;

    let on_spread_type_change = {
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let s = input.value();
            let spread_type = SpreadType::from_str(s.as_str()).unwrap();
            Dispatch::<DataState>::new().apply(ChangeSpreadTypeMessage(spread_type));
        })
    };

    let spread_type_options = SpreadType::iter()
    .map(|spread_type|{

        let selected = spread_type == user_data.spread_type;
        let name = description_state.descriptions.get(&spread_type).unwrap().dropdown_name;
        html!(  <option value={spread_type.repr()} {selected} disabled={false}> {name}  </option>
        )
    }

    ).collect_vec();

    let on_begin_click = {
        let navigator = navigator;
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<PromptsState>::new().apply(ShufflePromptsMessage);
            navigator.push(&Route::Question {});
        })
    };

    html! {

        <>
        <div class="site" style="">
            <div class="container" style=""  >
            <div class="sm-4 col" style="margin: auto; text-align: justify;">
            <div>
            <div class={"advanced-view-item"}  >
            <Logo clickable={true}/>
            </div>
            </div>
            <div>
            <div class={"advanced-view-item"}  >
            <GuideCarousel/>
            </div>
            <br/>
        <br/>
            </div>
            <div>
            <select onchange={on_spread_type_change} class="nice-button advanced-view-item" style="text-align: center;">
            {spread_type_options}
            </select>

            </div>
            <br/>
            <div>
            <p class="advanced-view-item" style="text-align: justify;">
                {description_state.get_guide_spread_text(&user_data)}
            </p>

            </div>
            <br/>
            <button onclick={on_begin_click} style="display: block;" class="nice-button advanced-view-item">{"Begin"}</button>
            <br/>
            <br/>
            </div>


            </div>
            </div>
        </>
    }
}
