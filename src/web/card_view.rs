use rand::rngs::ThreadRng;
use rand::RngCore;
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::{Route, ShareComponent};

#[derive(Properties, PartialEq)]
pub struct CardViewProps {
    pub meta: ImageMeta,
    pub show_description: bool,
    pub index: usize,
    pub description: ImageDescription,
    pub total_cards: usize,
    pub should_shake: bool,
    pub sign: Option<StarSign>,
}

#[function_component(CardView)]
pub fn card_view(props: &CardViewProps) -> Html {
    let toggle = Dispatch::<CardPageState>::new().apply_callback(|_| ToggleDescriptionMessage {});

    let onclick = {
        let guide = props.meta.guide.clone();
        let sign = props.sign.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |_e: MouseEvent| {
            navigator.replace(&Route::Restart {
                sign: sign.into(),
                guide,
            });
        })
    };

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");
    let top_card = props.index + 2 == props.total_cards;
    let show_description = if top_card {
        card_classes.push("top_card");

        if props.show_description {
            image_classes.push("image_greyed");
            true
        } else {
            false
        }
    } else {
        false
    };

    if props.should_shake {
        card_classes.push("card-shake");
    }

    let style = if props.index + 1 == props.total_cards {
        format!("transform:  translateX(15em) translateY(5em) rotateZ(-30deg); visibility: hidden; pointer-events: none;",)
    } else if props.index + 1 >= props.total_cards {
        let angle = match props.index % 4 {
            0 => 15 + ((props.index as isize) * -10),
            1 => -20 + ((props.index as isize) * 10),
            2 => 20 + ((props.index as isize) * -10),
            _ => -15 + ((props.index as isize) * 10),
        };

        let translate_x = match props.index % 4 {
            0 => 10,
            1 => -10,
            2 => 20,
            _ => -20,
        };

        let translate_y = match props.index % 4 {
            0 => 10,
            1 => -20,
            2 => -10,
            _ => 20,
        };

        format!(
            "transform:  translateX({}em) translateY({}em) rotateZ({}deg); visibility: hidden; pointer-events: none;",
            translate_x, translate_y, angle,
        )
    } else if top_card {
        let angle = 0;
        format!(
            "transform: rotateZ({}deg); transition-duration: 1s, 3s",
            angle
        )
    } else {
        let angle = match props.index % 4 {
            0 => 15 + ((props.index as isize) * -1),
            1 => -20 + ((props.index as isize) * 1),
            2 => 20 + ((props.index as isize) * -1),
            _ => -15 + ((props.index as isize) * 1),
        };

        let translate_x = match props.index % 4 {
            0 => 1,
            1 => -1,
            2 => 2,
            _ => -2,
        };

        let translate_y = match props.index % 4 {
            0 => 1,
            1 => 0,
            2 => -2,
            _ => -1,
        };

        format!(
            "transform: translateX({}em) translateY({}em)  rotateZ({}deg);s; pointer-events: none;",
            translate_x, translate_y, angle
        )
    };

    html! {

            <div class={card_classes} style = {style} >
            <div class="prophecy-back"> </div>
                    <img class={image_classes}  src={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone()) } onclick={toggle.clone()} />
                    {
                        if show_description{
                            html!{
                                <div class="image-overlay" style="pointer-events:none;">
                                <p class="image-overlay-text">
                                    <span>
                                    {props.description.representation.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {props.description.guidance.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {props.description.specific_guidance.clone()}
                                    </span>
                                </p>
                                <div class="row flex-spaces child-borders" style="margin-top: 3rem; margin-bottom: -3rem;">
                    <label class="paper-btn margin nice-button" for="modal-2"  style="pointer-events:auto;">{"Share"}</label>
                    <button class="margin nice-button" style="pointer-events:auto;" {onclick} >{"Continue"} </button>
                  </div>
                  <br/>
                  <input class="modal-state" id="modal-2" type="checkbox"/>
                  <div class="modal" style="pointer-events:auto;">
                    <label class="modal-bg" for="modal-2"></label>
                    <div class="modal-body">
                      <h4 class="modal-title">{"Share"}</h4>
                                <ShareComponent
                                title="intarot"
                                url={"https://www.intarot.com"}
                                text={props.description.full_description()}
                                media={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone())}>
                                </ShareComponent>

                    </div>
                  </div>
                                </div>
                            }
                        }
                        else{
                            html!{
                                <></>
                            }
                        }
                    }
        </div>
    }
}
