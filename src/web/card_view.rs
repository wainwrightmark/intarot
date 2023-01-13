use yew::prelude::*;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::{Route, ShareComponent};

#[derive(Properties, PartialEq)]
pub struct IndexedCardViewProps {
    pub index: usize,
}

#[derive(Properties, PartialEq)]
pub struct CardViewProps {
    pub top_card: bool,
    pub style: CardStyle,
    pub src_data: SrcData,
    pub description: Option<ImageDescription>,
    pub show_continue: bool,
}

#[function_component(CardView)]
pub fn card_view(props: &CardViewProps) -> Html {
    let state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();

    let toggle = Dispatch::<DataState>::new().apply_callback(|_| ToggleDescriptionMessage {});

    let on_continue_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<UserState>::new().apply(SetUsedBeforeMessage {});
            navigator.replace(&Route::Restart {});
        })
    };

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");

    let show_description = if props.top_card {
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

    let should_shake = props.top_card && !state.has_shown_description;

    if should_shake {
        card_classes.push("card-shake");
    }

    html! {

            <div class={card_classes} style = {props.style.get_style()} >
            <div class="prophecy-back"> </div>
                    <img class={image_classes}  src={props.src_data.src()} onclick={toggle.clone()} />
                    {
                        if show_description{
                            html!{
                                <div class="image-overlay" style="pointer-events:none;">
                                {
                                    if let Some(description) = props.description{
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
                        if props.show_continue{
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
                                url={props.src_data.share_url()}
                                text={props.description.map(|x|x.full_description()).unwrap_or_else(|| include_str!(r#"../text/opening_p1.txt"#).into())}
                                media={props.src_data.src()}>
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

#[function_component(IndexedCardView)]
pub fn indexed_card_view(props: &IndexedCardViewProps) -> Html {
    let descriptions_state = use_store_value::<ImageDescriptionState>();
    let metas_state = use_store_value::<ImageMetaState>();
    let state = use_store_value::<DataState>();
    let descriptions = &descriptions_state.descriptions;

    let metas = &metas_state.metas;

    let top_card = state.is_top_card(props.index);

    let style = get_style(props.index, state.as_ref());

    let meta = state.get_image_meta(props.index, metas);
    let description: Option<ImageDescription> = meta
        .and_then(|meta| descriptions.get(&(meta.guide, meta.card)))
        .map(|x| x.clone());

    let src_data = meta
        .map(|x| x.src_data())
        .unwrap_or_else(|| state.question_data.guide.ad_image_src());

    let show_continue = meta.is_none();

    html! {
        <CardView {top_card} {src_data} {style} {description} {show_continue} />
    }
}

fn get_style(index: usize, state: &DataState) -> CardStyle {
    if index == state.top_card_index + 1 {
        CardStyle {
            transform: Some(TransformStyle {
                translate_x: 15,
                translate_y: 5,
                rotate_z: -30,
            }),
            hidden: true,
            no_pointer_events: true,
        }
        //"transform:  translateX(15em) translateY(5em) rotateZ(-30deg); visibility: hidden; pointer-events: none;".to_string()
    } else if index > state.top_card_index + 1 {
        let rotate_z = match index % 4 {
            0 => 15 + ((index as i32) * -10),
            1 => -20 + ((index as i32) * 10),
            2 => 20 + ((index as i32) * -10),
            _ => -15 + ((index as i32) * 10),
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

        CardStyle {
            transform: Some(TransformStyle {
                translate_x,
                translate_y,
                rotate_z,
            }),
            hidden: true,
            no_pointer_events: true,
        }

        // format!(
        //     "transform:  translateX({translate_x}em) translateY({translate_y}em) rotateZ({angle}deg); visibility: hidden; pointer-events: none;",
        // )
    } else if index == state.top_card_index {
        CardStyle {
            hidden: false,
            no_pointer_events: false,
            transform: None,
        }
        // let angle = 0;
        // format!("transform: rotateZ({angle}deg); transition-duration: 1s, 3s")
    } else {
        let rotate_z = match index % 4 {
            0 => 15 + -(index as i32),
            1 => -20 + (index as i32),
            2 => 20 + -(index as i32),
            _ => -15 + (index as i32),
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
        CardStyle {
            hidden: false,
            no_pointer_events: true,
            transform: Some(TransformStyle {
                translate_x,
                translate_y,
                rotate_z,
            }),
        }

        // format!(
        //     "transform: translateX({translate_x}em) translateY({translate_y}em)  rotateZ({angle}deg);s; pointer-events: none;"
        // )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TransformStyle {
    pub translate_x: i32,
    pub translate_y: i32,
    pub rotate_z: i32,
}

impl TransformStyle {
    pub fn get_style(&self) -> String {
        let translate_x = self.translate_x;
        let translate_y = self.translate_y;
        let rotate_z = self.rotate_z;
        let transform = format!("transform: translateX({translate_x}em) translateY({translate_y}em)  rotateZ({rotate_z}deg); ");

        transform
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CardStyle {
    pub hidden: bool,
    pub no_pointer_events: bool,
    pub transform: Option<TransformStyle>,
}

impl CardStyle {
    pub fn get_style(&self) -> String {
        let mut a = "".to_string();
        if let Some(transform) = self.transform {
            a.push_str(transform.get_style().as_str());
        }
        if self.hidden {
            a.push_str("visibility: hidden;")
        }
        if self.no_pointer_events {
            a.push_str("pointer-events: none;")
        }
        a
    }
}
