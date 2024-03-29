use itertools::Itertools;
use std::str::FromStr;
use strum::IntoEnumIterator;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::{
    data::prelude::*,
    state::{
        nagging_state::{AdvancedPageVisitMessage, NaggingState},
        prelude::*,
        prompts_state::PromptsState,
    },
    web::prelude::*,
};

#[derive(Properties, PartialEq)]
pub struct AdvancedProps {}

#[function_component(AdvancedView)]
pub fn advanced_view(_props: &AdvancedProps) -> Html {
    use_effect_once(|| {
        scroll_to_top();
        spawn_local(async {
            Dispatch::<NaggingState>::new()
                .apply_future(AdvancedPageVisitMessage {})
                .await;
        });

        || ()
    });

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

            <div class="container" style="overflow-x: hidden"  >
            <div class="contained col min-width-contained">

            <div>
                <Logo clickable={true} invertible={true}/>
            </div>
            <div>
            <div class={"advanced-view-item"}  >
            <h5 class="advanced-view-header">{"Choose Your Guide"}</h5>
            <GuideCarousel/>
            </div>
            <br/>
            </div>
            <div>
            <h5 class="advanced-view-header">{"Choose Your Reading"}</h5>
            <select onchange={on_spread_type_change} class="nice-button advanced-view-item advanced-view-button" style="text-align: center;">
            {spread_type_options}
            </select>

            </div>
            <br/>
            <div>
            <p class="advanced-view-item" style="text-align: left;">
                {description_state.get_guide_spread_text(&user_data)}
            </p>

            </div>
            <br/>
            <br/>
            <button onclick={on_begin_click} style="display: block;" class="nice-button advanced-view-item advanced-view-button">{"Begin"}</button>            
            <br/>
            <br/>
            <div class={"advanced-view-item"}  >
                <SocialIcons />

            </div>
            </div>


            </div>
            </div>
        </>
    }
}
