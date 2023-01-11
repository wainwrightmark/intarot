use yew::prelude::*;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::state::prelude::*;
use crate::web::prelude::{Route, ShareComponent};

#[derive(Properties, PartialEq)]
pub struct CardViewProps {
    pub index: usize,
}

#[function_component(CardView)]
pub fn card_view(props: &CardViewProps) -> Html {
    // log::info!("Card View {}", props.index);
    let descriptions_state = use_store_value::<ImageDescriptionState>();
    let metas_state = use_store_value::<ImageMetaState>();
    let state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();
    let Some(descriptions) = descriptions_state.descriptions.as_ref() else{
        // log::info!("No descriptions");
        return html!();
    };

    let Some(metas) = metas_state.metas.as_ref() else{
        // log::info!("No Metas");
        return html!();
    };

    let top_card = state.is_top_card(props.index);
    // log::info!("Showing Card View index:{} top:{top_card}", props.index);

    let toggle = Dispatch::<DataState>::new().apply_callback(|_| ToggleDescriptionMessage {});

    let on_continue_click = {
        Callback::from(move |_e: MouseEvent| {
            navigator.replace(&Route::Restart {});
        })
    };

    let style = get_style(props.index, state.as_ref());

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");

    let show_description = if top_card {
        card_classes.push("top_card");

        if state.show_description {
            image_classes.push("image_greyed");
            true
        } else {
            false
        }
    } else {
        false
    };

    let should_shake = top_card && !state.has_shown_description;

    if should_shake {
        card_classes.push("card-shake");
    }

    let meta = state.get_image_meta(props.index, &metas);
    let description = meta.and_then(|meta| descriptions.get(&(meta.guide, meta.card)));

    let id =meta.map(|x|x.id).unwrap_or(state.user_data.guide.ad_image_id());

    html! {

            <div class={card_classes} style = {style} >
            <div class="prophecy-back"> </div>
                    <img class={image_classes}  src={format!("https://drive.google.com/uc?export=view&id={}",id) } onclick={toggle.clone()} />
                    {
                        if show_description{
                            html!{
                                <div class="image-overlay" style="pointer-events:none;">
                                {
                                    if let Some(description) = description{
                                        html!{
                                            <p class="image-overlay-text">
                                    <span>
                                    {description.representation.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {description.guidance.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {description.specific_guidance.clone()}
                                    </span>
                                </p>
                                        }
                                    }else{
                                        html!{
                                            <></>
                                        }
                                    }
                                }



                                <div class="row flex-spaces child-borders" style="margin-top: 3rem; margin-bottom: -3rem; flex-direction: column;">
                    <label class="paper-btn margin nice-button" for="modal-2"  style="pointer-events:auto;">{"Share"}</label>
                    <br/>
                    {
                        if meta.is_none(){
                            html!{
                                <button class="margin nice-button" style="pointer-events:auto;" onclick={on_continue_click} >{"Continue"} </button>
                            }
                        }else{
                            html!{
                                <></>
                            }
                        }
                    }
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
                                text={description.map(|x|x.full_description()).unwrap_or_else(|| include_str!(r#"../text/opening_p1.txt"#).into())}
                                media={format!("https://drive.google.com/uc?export=view&id={}", id)}>
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

fn get_style(index: usize, state: &DataState) -> String {
    if index == state.top_card_index + 1 {
        "transform:  translateX(15em) translateY(5em) rotateZ(-30deg); visibility: hidden; pointer-events: none;".to_string()
    } else if index > state.top_card_index + 1 {
        let angle = match index % 4 {
            0 => 15 + ((index as isize) * -10),
            1 => -20 + ((index as isize) * 10),
            2 => 20 + ((index as isize) * -10),
            _ => -15 + ((index as isize) * 10),
        };

        let translate_x = match index % 4 {
            0 => 10,
            1 => -10,
            2 => 20,
            _ => -20,
        };

        let translate_y = match index % 4 {
            0 => 10,
            1 => -20,
            2 => -10,
            _ => 20,
        };

        format!(
            "transform:  translateX({translate_x}em) translateY({translate_y}em) rotateZ({angle}deg); visibility: hidden; pointer-events: none;",
        )
    } else if index == state.top_card_index {
        let angle = 0;
        format!("transform: rotateZ({angle}deg); transition-duration: 1s, 3s")
    } else {
        let angle = match index % 4 {
            0 => 15 + -(index as isize),
            1 => -20 + (index as isize),
            2 => 20 + -(index as isize),
            _ => -15 + (index as isize),
        };

        let translate_x = match index % 4 {
            0 => 1,
            1 => -1,
            2 => 2,
            _ => -2,
        };

        let translate_y = match index % 4 {
            0 => 1,
            1 => 0,
            2 => -2,
            _ => -1,
        };

        format!(
            "transform: translateX({translate_x}em) translateY({translate_y}em)  rotateZ({angle}deg);s; pointer-events: none;"
        )
    }
}
