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
        // <Carousel/>
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <SignDropdown/>

        </div>
        <div class="sm-4 col" style="margin: auto; width: 100em;">
        <ImageView/>

        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <RerollButton/>
        </div>
            </div>
        </div>
    }
}

#[function_component(RerollButton)]
pub fn reroll_button() -> Html {
    let reroll =
        Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, _: MouseEvent| s.reroll());

    html! {
        <button id="reroll-button" aria-label="Reroll" onclick={reroll}>{"Draw Another"}</button>
    }
}

#[function_component(Carousel)]
pub fn carousel() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let select_previous =
        Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, _: MouseEvent| {
            s.soothsayer = s.soothsayer.map(|ss| ss.previous()).flatten();
        });

    let select_next = Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, _: MouseEvent| {
        s.soothsayer = s
            .soothsayer
            .map_or(Some(Soothsayer::Madame), |ss| ss.next());
    });

    let items = Soothsayer::iter()
    .map(|ss|{
        let selected = image_state.soothsayer == Some(ss) || image_state.soothsayer.is_none() && ss == Soothsayer::Madame;
        let classes = if selected {classes!("carousel-item", "carousel-item-visible")} else{classes!("carousel-item", "carousel-item-hidden")};
        html!(
            <div class={classes}>
                <h2 class="soothsayer-name">{ss.name()}</h2>
                <img class="soothsayer-image" src={format!("https://drive.google.com/uc?export=view&id={}", ss.image_id()) } 
                     alt={ss.name()} />
            </div>
        )
    }).collect_vec();

    html! {
        <div class="carousel">
            {items}

            <div class="carousel-actions">
                <button id="carousel-button-prev" aria-label="Previous" onclick={select_previous}></button>
                <button id="carousel-button-next" aria-label="Next" onclick={select_next}></button>
            </div>
        </div>
    }
}

#[function_component(ImageView)]
pub fn image_view() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let image_meta_option = image_state.get_image_meta();

    if let Some(img) = image_meta_option {
        html! {
                    <div class="grid">
                    <div class="prophecy-image-container">
                        <div class="prophecy-image-text">{img.card.name()}</div>
                        <img class="prophecy-image" src={format!("https://drive.google.com/uc?export=view&id={}", img.id.clone()) }/>


                        </div>
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
