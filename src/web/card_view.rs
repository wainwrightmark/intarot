use yew::prelude::*;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::Route;
use crate::web::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardViewProps {
    pub top_card: bool,
    pub style: CardStyle,
    pub src_data: SrcData,
    pub description: Option<ImageDescription>,
    pub show_extra_buttons: bool,
    pub slot: Option<&'static str>,
    pub description_layout: DescriptionLayout,
}

#[function_component(CardView)]
pub fn card_view(props: &CardViewProps) -> Html {
    let data_state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();

    let guide = data_state.question_data.guide;
    let toggle = Dispatch::<DataState>::new().apply_callback(|_| ToggleDescriptionMessage {});
    let description_layout = props.description_layout;

    let on_continue_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickAnotherReading));
            navigator.replace(&Route::Advanced {});
        })
    };

    let on_survey_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickSurvey));
            open_link_in_new_tab("https://docs.google.com/forms/d/e/1FAIpQLSep7npbKOtYcA_ruRFK8ByHz0Zjl_7-gp6YQ3XPhJ_QHLgw4w/viewform".to_string());
        })
    };
    let on_discord_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickDiscord));
            open_link_in_same_tab("https://discord.gg/eRm5YdMNAw".to_string());
        })
    };

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

    let share_text = match props.description {
        Some(d) => d.description_sections(&description_layout).first().unwrap(),
        None => include_str!(r#"../text/opening_p1.txt"#),
    };

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

                    <img class={image_classes}  src={props.src_data.src()} onclick={toggle.clone()} />
                    {
                        if show_description{
                            html!{
                                <div class="image-overlay" style="pointer-events:none;">
                                {
                                    if props.show_extra_buttons{
                                        html!{
                                            <div class =" buttons-grid" style="margin-top:1em;">
                                            <div class="row flex-spaces child-borders" style="flex-direction: column; margin-bottom:0">
                                            <ShareButton label="Readings can be mysterious, why not share and discuss yours?" for_id="share_modal" src_data={props.src_data}/>
                                            </div>
                                            <button class="margin nice-button card-button" style="pointer-events:auto;" onclick={on_survey_click}  >{"Want to help improve intarot? Please fill out our 2 minute survey"} </button>
                                            <button class="margin nice-button card-button" style="pointer-events:auto;" onclick={on_continue_click} >{"Do another reading"} </button>
                                            <button class="margin nice-button card-button" style="pointer-events:auto;" onclick={on_discord_click}  >{"Discuss on Discord"} </button>
                                            </div>

                                        }
                                    }else{
                                        html!{
                                            <div style="position:absolute; top: 90%; left:50%; transform: translateX(-50%);" >
                                                <ShareButton for_id="share_modal" src_data={props.src_data} />
                                            </div>
                                        }
                                    }

                                }
                                <ShareModal src_data={props.src_data} share_text={share_text} id="share_modal"/>
                                {
                                    if let Some(description) = props.description{

                                        let sections = description.description_sections(&description_layout).iter().map(|x| html!(
                                           <>
                                           <span>
                                           {x}
                                           </span>
                                           <br/>
                                           <br/>
                                           </>
                                        )).collect::<Html>();
                                        html!{
                                            <p class="image-overlay-text">
                                    {sections}
                                </p>
                                        }
                                    }else{
                                        html!{
                                            <></>
                                        }
                                    }
                                }

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
                            html!{
                                <SlotView {slot} {guide}/>
                            }
                        }
                        else{
                            html!(<></>)
                        }
                    }
        </div>

    }
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SlotProperties {
    pub slot: &'static str,
    pub guide: Guide,
}

#[function_component(SlotView)]
pub fn slot_view(props: &SlotProperties) -> Html {
    html! {
        <div class="slot" style={format!("background-image: linear-gradient({}, {});", props.guide.primary_color(), props.guide.secondary_color())}>
            {props.slot}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IndexedCardViewProps {
    pub index: u8,
}

#[function_component(IndexedCardView)]
pub fn indexed_card_view(props: &IndexedCardViewProps) -> Html {
    let image_descriptions_state = use_store_value::<ImageDescriptionState>();
    let metas_state = use_store_value::<ImageMetaState>();
    let data_state = use_store_value::<DataState>();
    let spread_descriptions_state = use_store_value::<SpreadDescriptionState>();

    let slot = spread_descriptions_state.try_get_slot(&data_state.question_data, props.index);
    let description_layout = spread_descriptions_state
        .try_get_layout(&data_state.question_data, props.index)
        .unwrap_or_default();
    let descriptions = &image_descriptions_state.descriptions;

    let metas = &metas_state.metas;

    let top_card = data_state.is_top_card(props.index);

    let style = get_style(props.index, data_state.as_ref());

    let meta = data_state.get_image_meta(props.index, metas);
    let description: Option<ImageDescription> = meta
        .and_then(|meta| descriptions.get(&(meta.guide, meta.card)))
        .copied();

    let src_data = meta
        .map(|x| x.src_data())
        .unwrap_or_else(|| data_state.spread_src(metas));

    let show_extra_buttons = meta.is_none();

    html! {
        <CardView {top_card} {src_data} {style} {description} {show_extra_buttons} {slot} {description_layout} />
    }
}

fn get_style(index: u8, state: &DataState) -> CardStyle {
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
    } else if index == state.top_card_index {
        CardStyle {
            hidden: false,
            no_pointer_events: false,
            transform: None,
        }
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
        let translate_x = -self.translate_x;
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
