use itertools::Itertools;

use std::str::FromStr;

use strum::{IntoEnumIterator, EnumProperty};
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::web::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="site">
            <div class="container" >
            <Content/>
            </div>
        </div>
    }
}

#[function_component(Content)]
pub fn content() -> Html {
    let (image_state, _) = use_store::<ImageState>();

    match image_state.as_ref() {
        ImageState::OpeningPage(_) => html!(<FrontPage />),
        ImageState::SoothsayerPage(_, _) => html!(<SoothsayerPage />),
        ImageState::CardPage(_, _, _) => html!(<ImagePage />),
    }
}

#[function_component(ProceedButton)]
pub fn proceed_button() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let proceed =
        Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, _: MouseEvent| s.proceed());

    html! {
        <button id="proceed-button" aria-label="Proceed" disabled={!image_state.can_proceed()} onclick={proceed}>{"Proceed"}</button>
    }
}

#[function_component(ResetButton)]
pub fn reset_button() -> Html {
    let reset =
        Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, _: MouseEvent| s.reset());

    html! {
        <button id="proceed-button" aria-label="Reset" onclick={reset}>{"Reset"}</button>
    }
}

#[function_component(FrontPage)]
pub fn front_page() -> Html {
    html! {
        <div>
        <div class="sm-4 col" style="margin: auto; width: 100em;">

        <h3 style="color: gold; text-align: center;">
        {"The Eighth Arcana"}
        </h3>
        <p>
        {"The Eighth Arcana brings tarot into the space that lies beyond the limits of the world. Our lives are lost in wandering, the deterministic patterns of our destiny masked in the grey fog of randomness. Our soothsayers can help reveal the path your feet should tread."}
        </p>
        <p>
        {"Each of our soothsayers will offer you an interpretation of the card they draw for you, but their portentous drawings may contain the seed of further truth - only you will recognise the signs that fate has chosen for you."}
        </p>
            </div>
            <div class="sm-4 col" style="margin: auto; width: 10em;">
            <SignDropdown />
            </div>
            <div class="sm-4 col" style="margin: auto; width: 10em;">
            <ProceedButton />
            </div>
        </div>
    }
}

#[function_component(SoothsayerPage)]
pub fn soothsayer_page() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let select_previous = Dispatch::<ImageState>::new()
        .reduce_mut_callback_with(|s, _: MouseEvent| s.previous_soothsayer());

    let select_next = Dispatch::<ImageState>::new()
        .reduce_mut_callback_with(|s, _: MouseEvent| s.next_soothsayer());

    let soothsayer = image_state.get_soothsayer();

    let items = Soothsayer::iter()
    .map(|ss|{
        let selected = ss == soothsayer;
        let classes = if selected {classes!("carousel-item", "carousel-item-visible")} else{classes!("carousel-item", "carousel-item-hidden")};
        html!(
            <div class={classes}>
                <h5 class="soothsayer-name" style="text-align: center;">{ss.name()}</h5>
                <img class="soothsayer-image" src={format!("https://drive.google.com/uc?export=view&id={}", ss.image_id()) } 
                     alt={ss.name()} />
                    <p class="soothsayer-text" >
                    {ss.get_str("description")}
                    </p>
            </div>
        )
    }).collect_vec();

    html! {
        <>
        <div class="sm-4 col" style="margin: auto; width: 100em; padding:unset;">
        <div class="carousel">
            {items}

            <div class="carousel-actions">
                {if image_state.can_previous_soothsayer(){
                    html!{
                        <button id="carousel-button-prev" aria-label="Previous" onclick={select_previous}></button>
                    }
                } else{html!{<></>}}}
                
                {if image_state.can_next_soothsayer(){
                    html!{
                        <button id="carousel-button-next" aria-label="Next"  disabled={!image_state.can_next_soothsayer()} onclick={select_next}></button>
                    }
                } else{html!{<></>}}}
                
                
            </div>
        </div>
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <ProceedButton/>
        </div>
        </>
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

#[function_component(ImagePage)]
pub fn image_page() -> Html {
    html!(
        <>
        <div class="sm-4 col" style="margin: auto; width: 100em;">
        <ImageView />
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <RerollButton />
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <ResetButton />
        </div>
        </>
    )
}

#[function_component(ImageView)]
pub fn image_view() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let image_meta_option = image_state.get_image_meta();

    if let Some(img) = image_meta_option {
        html! {
                    <div class="grid">
                    <div class="prophecy-image-container">
                        <div class="prophecy-image-card" key={img.id.clone()}>
                            <img class="prophecy-image-front"  src={format!("https://drive.google.com/uc?export=view&id={}", img.id.clone()) }/>
                            <div class="prophecy-image-back"/>
                            <div class="prophecy-image-ghost"/>
                        </div>

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
            s.set_star_sign(Some(sign));
        } else {
            s.set_star_sign(None);
        }
    });

    let current_sign = use_selector::<ImageState, _, _>(|x| x.get_sign());

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
        <option value="" disabled={true} selected={current_sign.is_none()}>{"Pick your Star Sign"}</option>

            {options}
        </select>
    )
}

// #[function_component(SoothsayerSelect)]
// pub fn soothsayer_dropdown() -> Html {
//     let onchange = Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, e: Event| {
//         let input: HtmlSelectElement = e.target_unchecked_into();
//         let value = input.value();
//         if let Ok(soothsayer) = Soothsayer::from_str(&value) {
//             s.soothsayer = Some(soothsayer);
//         } else {
//             s.soothsayer = None;
//         }
//     });

//     let current_soothsayer = use_selector::<ImageState, _, _>(|x| x.soothsayer);

//     let options = Soothsayer::iter()
//         .into_iter()
//         .map(|soothsayer| {
//             let selected = Some(soothsayer) == *current_soothsayer;
//             html!(  <option value={soothsayer.repr()} {selected}>{soothsayer.name()}</option>
//             )
//         })
//         .collect_vec();

//     html!(
//         <select {onchange}  style="background: var(--main-background)">
//         <option value="" disabled={true} selected={current_soothsayer.is_none()}>{"Soothsayer"}</option>

//             {options}
//         </select>
//     )
// }
