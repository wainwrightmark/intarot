use itertools::Itertools;

use std::str::FromStr;

use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::web::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="site">
            <div class="container-sm" >
        //         <div class="card" style="width: 20rem;">
        // <img src="https://picsum.photos/768" alt="Card example image"/>

        // <div class="card-body">
        //   <h4 class="card-title">{"Maledictus Andromachus"}</h4>
        //   <button>{"Choose Maledictus"}</button>
        // </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <SoothsayerSelect/>
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <SignDropdown/>
        </div>
        <div class="sm-4 col" style="margin: auto; width: 100em;">
        <ImageView/>
        </div>
            </div>
        </div>
    }
}

#[function_component(ImageView)]
pub fn image_view() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let image_meta_option = image_state.get_image_meta();

    if let Some(image_meta) = image_meta_option {
        let images = image_meta
            
            .iter()
            .map(|img| {
                html! {
                    <img style="border-radius: 20px; border-color: black" src={format!("https://drive.google.com/uc?export=view&id={}", img.id.clone()) }/>
                }
            }).take(1)
            .collect_vec();

        html! {
                    <div class="grid">
          {images}
        </div>
                }
    } else {
        html!(<></>)
    }
}

#[function_component(SignDropdown)]
pub fn sign_dropdown() -> Html {
    let onchange = Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        if let Ok(sign) = StarSign::from_str(&value) {
            s.sign = Some(sign);
        } else {
            s.sign = None;
        }
    });

    let current_sign = use_selector::<ImageState, _, _>(|x| x.sign);

    let options = StarSign::iter()
        .into_iter()
        .map(|sign| {
            let selected = Some(sign) == *current_sign;
            html!(  <option value={sign.repr()} {selected}>{sign.name()}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange} style="background: var(--main-background)">
        <option value="" disabled={true} selected={current_sign.is_none()}>{"Star Sign"}</option>

            {options}
        </select>
    )
}

#[function_component(SoothsayerSelect)]
pub fn soothsayer_dropdown() -> Html {
    let onchange = Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        if let Ok(soothsayer) = Soothsayer::from_str(&value) {
            s.soothsayer = Some(soothsayer);
        } else {
            s.soothsayer = None;
        }
    });

    let current_soothsayer = use_selector::<ImageState, _, _>(|x| x.soothsayer);

    let options = Soothsayer::iter()
        .into_iter()
        .map(|soothsayer| {
            let selected = Some(soothsayer) == *current_soothsayer;
            html!(  <option value={soothsayer.repr()} {selected}>{soothsayer.name()}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange}  style="background: var(--main-background)">
        <option value="" disabled={true} selected={current_soothsayer.is_none()}>{"Soothsayer"}</option>

            {options}
        </select>
    )
}
