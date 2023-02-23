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
}

#[function_component(TarotCard)]
pub fn tarot_card(props: &TarotCardProps) -> Html {
    let data_state = use_store_value::<DataState>();

    let guide = data_state.question_data.guide;
    let toggle = Dispatch::<DataState>::new().apply_callback(|_| ToggleDescriptionMessage {});
    let description_layout = props.description_layout;

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");

    let show_description = if props.top_card {
        card_classes.push("top_card");

        if data_state.show_description {
            image_classes.push("image_greyed");
            true
        } else {
            false
        }
    } else {
        false
    };

    let should_shake = props.top_card && !data_state.has_shown_description;

    if should_shake {
        card_classes.push("card-shake");
    }

    let share_text = *props
        .description
        .description_sections(&description_layout)
        .first()
        .unwrap();

        let sections = props.description.description_sections(&description_layout).iter().map(|x| html!(
            <>
            <span>
            {x}
            </span>
            <br/>
            <br/>
            </>
         )).collect::<Html>();

    html! {

            <div class={card_classes} style = {props.style.get_style()} >
            {
                if props.top_card{
                    html!(<div class="prophecy-back"/>)
                }
                else{
                    html!(<></>)
                }
            }

                    <img class={image_classes}  src={props.src_data.image.src()} onclick={toggle.clone()} />
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
                            let hide = data_state.show_description;
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
