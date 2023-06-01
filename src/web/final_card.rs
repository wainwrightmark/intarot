use num_traits::FromPrimitive;
use yew::prelude::*;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::Route;
use crate::web::prelude::*;

use super::card_view::CardStyle;

#[derive(Properties, PartialEq)]
pub struct FinalCardProps {
    pub top_card: bool,
    pub style: CardStyle,
    pub src_data: SrcData,
}

#[function_component(FinalCard)]
pub fn final_card(props: &FinalCardProps) -> Html {
    let data_state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();

    let guide = data_state.question_data.guide;

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

    let on_coffee_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickCoffee));
            open_link_in_new_tab("https://ko-fi.com/intarot".to_string());
        })
    };

    let mut card_classes = classes!("prophecy-card");
    let image_classes = classes!("prophecy-image");

    if props.top_card {
        card_classes.push("top_card");
    };

    let share_text = get_share_text(&props.src_data);
    let src_data = props.src_data.clone();

    let img_style = format!(
        "background: linear-gradient(30deg, {}, {});",
        guide.primary_color(),
        guide.secondary_color()
    );
    let buttons_div_style = if props.top_card {
        "opacity: 1; pointer-events:auto;"
    } else {
        "opacity: 0; pointer-events:none;"
    };

    html! {

            <div class={card_classes} style = {props.style.get_style()} >

                <img class={image_classes} style={img_style} />

                <div class="image-overlay" style="pointer-events:none;">
                    <div class ="final-card-grid">
                        <Logo clickable={false} invertible={false}/>
                        <div class="final-card-buttons" style={buttons_div_style}>
                                <button class="nice-button card-button" onclick={on_continue_click} >{"Do another reading"} </button>
                                <ShareButton label="Share your reading" {share_text} {src_data}/>
                                <button class="nice-button card-button" onclick={on_survey_click}  >{"Do our quick survey"} </button>
                                <button class="nice-button card-button" onclick={on_coffee_click}  >{"Buy us a coffee"} </button>
                            </div>

                    </div>

                </div>



        </div>

    }
}

fn get_share_text(data: &SrcData) -> String {
    let Some(spread_share) = &data.spread_option else{
        return "The tarot app that combines AI-generated insight with self-reflection.".to_string();
    };

    let mut text = spread_share
        .question_data
        .spread_type
        .long_name()
        .to_string();

    if spread_share.question_data.spread_type.total_cards() <= 7 {
        text.push_str("\r\n");
        for card_index in (0..spread_share.question_data.spread_type.total_cards()).rev() {
            let card: Card = spread_share.perm.element_at_index(card_index, |x| {
                Card::from_u8(x).expect("Could not make card from u8")
            });
            text.push_str(card.name());
            text.push_str("\r\n");
        }
    }

    text
}
