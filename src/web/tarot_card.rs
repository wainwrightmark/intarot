use yew::prelude::*;

use yewdux::prelude::*;

use crate::data::image_data::ImageData;
use crate::data::prelude::*;
use crate::state::preferences_state::CardShakeState;
use crate::state::prelude::*;
use crate::web::card_view::SlotView;
use crate::web::prelude::*;

use super::card_view::CardStyle;

#[derive(Properties, PartialEq)]
pub struct TarotCardProps {
    pub top_card: bool,
    pub style: CardStyle,
    pub src_data: SrcData,
    pub slot: Option<&'static str>,
    pub description_layout: DescriptionLayout,
    pub face_up: bool,
    pub card: Card,
    pub guide: Guide,
}

#[function_component(TarotCard)]
pub fn tarot_card(props: &TarotCardProps) -> Html {
    let (layout, card, guide) = (props.description_layout, props.card, props.guide);
    let descriptions = use_selector(move|x: &ImageDescriptionState|  {
        x.get_layout_sections(&layout, &card, &guide)
    });
    let data_state = use_store_value::<DataState>();
    let shake_state = use_store_value::<CardShakeState>();
    let guide = props.guide;

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");

    if props.top_card {
        card_classes.push("top_card"); //needed for animate animations
        if data_state.show_description {
            image_classes.push("image_greyed");
        }
    }

    let show_description = props.top_card && props.face_up && data_state.show_description;

    let should_shake =
        props.top_card && shake_state.enabled && !data_state.has_shown_description && props.face_up;

    if should_shake {
        card_classes.push("card-shake");
    }

    let share_text = descriptions.first().cloned().unwrap_or_default();

    let sections =
        descriptions.iter()
        .map(|x| {
            html!(
               <>
               <span>
               {x}
               </span>
               <br/>
               <br/>
               </>
            )
        })
        .collect::<Html>();

    let card_back_src = guide.card_back();
    let frame_src = ImageData::placeholder(guide, props.card).src();

    let back_style = format!("background: {};", guide.primary_color());
    html! {

            <div class={card_classes} style={props.style.get_style()}  >
                <div class="prophecy-back">
                    <img class="prophecy-image" src={card_back_src} style={back_style} alt="the back of a tarot card" />
                </div>

                <div class="prophecy-middle">
                    <img class="prophecy-image" src={frame_src}  />
                </div>

                <img class={image_classes} style="cursor: pointer;" src={props.src_data.image.src()} alt={props.src_data.image.alt()} />
                {
                    if show_description{
                        let src_data = props.src_data.clone();
                        html!{
                            <div class="image-overlay" style="pointer-events:none;">
                            <div style="position:absolute; top: 90%; left:50%; transform: translateX(-50%);" >
                            <ShareButton {share_text}  {src_data}/>
                        </div>
                        <p class="image-overlay-text">
                            {sections}
                        </p>

                            </div>
                        }
                    }
                    else{
                        html!{
                            <></>
                        }
                    }
                }
                {
                    if let Some(slot) = props.slot.filter(|_|props.top_card){
                        let hide = data_state.show_description  || !props.face_up;
                        html!{
                            <SlotView {slot} {guide} {hide}/>
                        }
                    }
                    else{
                        html!(<></>)
                    }
                }


        </div>

    }
}
