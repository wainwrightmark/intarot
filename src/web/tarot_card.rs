use yew::prelude::*;

use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::card_view::SlotView;
use crate::web::prelude::*;

use super::card_view::CardStyle;

#[derive(Properties, PartialEq)]
pub struct TarotCardProps {
    pub top_card: bool,
    pub style: CardStyle,
    pub src_data: SrcData,
    pub description: ImageDescription,
    pub slot: Option<&'static str>,
    pub description_layout: DescriptionLayout,
    pub face_up: bool,
}

#[function_component(TarotCard)]
pub fn tarot_card(props: &TarotCardProps) -> Html {
    let data_state = use_store_value::<DataState>();

    let guide = data_state.question_data.guide;
    let description_layout = props.description_layout;

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");

    if props.top_card {
        card_classes.push("top_card"); //needed for animate animations
        if data_state.show_description {
            image_classes.push("image_greyed");
        }
    }

    let show_description = props.top_card && props.face_up && data_state.show_description;

    let should_shake = props.top_card && !data_state.has_shown_description && props.face_up;

    if should_shake {
        card_classes.push("card-shake");
    }

    let share_text = *props
        .description
        .description_sections(&description_layout)
        .first()
        .unwrap();

    let sections = props
        .description
        .description_sections(&description_layout)
        .iter()
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
    let frame_src = guide.frame();

    let back_style = format!("background: {};", guide.primary_color());
    html! {

            <div class={card_classes} style={props.style.get_style()}  >



                <div class="prophecy-back">
                    <img class="prophecy-image" src={card_back_src} style={back_style} />
                </div>

                <div class="prophecy-middle">
                    <img class="prophecy-image" src={frame_src} style={"background: white;"} />
                </div>





                <img class={image_classes} style="cursor: pointer;"  src={props.src_data.image.src()} />
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
